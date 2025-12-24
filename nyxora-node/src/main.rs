use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

#[derive(Parser)]
#[command(name = "nyxora-node")]
#[command(about = "A validator node for the Nyxora quantum-hybrid cryptocurrency")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.json")]
    config: String,

    /// Run as validator node
    #[arg(long)]
    validator: bool,

    /// Port to run the node on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeConfig {
    pub address: String,
    pub stake_amount: u64,
    pub is_validator: bool,
    pub quantum_enabled: bool,
    pub peers: Vec<String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            address: "Q123456789012345678901234567890123456789".to_string(),
            stake_amount: 1000,
            is_validator: false,
            quantum_enabled: false,
            peers: vec!["127.0.0.1:8081".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeState {
    pub config: NodeConfig,
    pub block_height: u64,
    pub peers: Vec<String>,
    pub status: String,
}

// Simplified consensus structs for the node
mod consensus {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Validator {
        pub address: String,
        pub stake: u64,
        pub last_block_proposed: u64,
        pub uptime: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Block {
        pub index: u64,
        pub timestamp: u64,
        pub prev_hash: String,
        pub transactions: Vec<String>,
        pub proposer: String,
        pub hash: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct QuantumProof {
        pub circuit_descriptor: String,
        pub measurement_results: Vec<u8>,
        pub proof_artifact: String,
        pub qubit_count: usize,
        pub validator_address: String,
        pub timestamp: u64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HybridBlock {
        pub pos_block: Block,
        pub quantum_proofs: Vec<QuantumProof>,
        pub hybrid_hash: String,
    }

    #[derive(Clone)]
    pub struct PoSConsensus {
        pub validators: HashMap<String, Validator>,
        pub total_stake: u64,
        pub current_block: u64,
    }

    impl PoSConsensus {
        pub fn new() -> Self {
            PoSConsensus {
                validators: HashMap::new(),
                total_stake: 0,
                current_block: 0,
            }
        }

        pub fn register_validator(&mut self, address: String, initial_stake: u64) {
            self.validators.insert(
                address.clone(),
                Validator {
                    address,
                    stake: initial_stake,
                    last_block_proposed: 0,
                    uptime: 100.0,
                }
            );
            self.total_stake += initial_stake;
        }

        pub fn stake(&mut self, address: &str, amount: u64) -> bool {
            if let Some(validator) = self.validators.get_mut(address) {
                validator.stake += amount;
                self.total_stake += amount;
                true
            } else {
                false
            }
        }
    }

    #[derive(Clone)]
    pub struct PoQConsensus {
        pub validator_scores: HashMap<String, u64>,
    }

    impl PoQConsensus {
        pub fn new() -> Self {
            PoQConsensus {
                validator_scores: HashMap::new(),
            }
        }
    }

    #[derive(Clone)]
    pub struct HybridConsensus {
        pub pos: PoSConsensus,
        pub poq: PoQConsensus,
    }

    impl HybridConsensus {
        pub fn new() -> Self {
            HybridConsensus {
                pos: PoSConsensus::new(),
                poq: PoQConsensus::new(),
            }
        }

        pub fn register_validator(&mut self, address: String, initial_stake: u64) {
            self.pos.register_validator(address, initial_stake);
        }

        pub fn stake(&mut self, address: &str, amount: u64) -> bool {
            self.pos.stake(address, amount)
        }
    }
}

struct NyxoraNode {
    state: Arc<Mutex<NodeState>>,
    consensus: Arc<Mutex<consensus::HybridConsensus>>,
}

impl NyxoraNode {
    fn new(config: NodeConfig) -> Self {
        let mut consensus = consensus::HybridConsensus::new();

        // Register this node as a validator if configured as one
        if config.is_validator {
            consensus.pos.register_validator(config.address.clone(), config.stake_amount);
        }

        NyxoraNode {
            state: Arc::new(Mutex::new(NodeState {
                config,
                block_height: 0,
                peers: vec![],
                status: "running".to_string(),
            })),
            consensus: Arc::new(Mutex::new(consensus)),
        }
    }

    async fn start_server(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let addr = ([127, 0, 0, 1], port).into();

        let node = self.clone_for_hyper();

        let make_svc = make_service_fn(move |_conn| {
            let node = node.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let node = node.clone();
                    handle_request(req, node)
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        println!("Nyxora node running on http://{}", addr);

        server.await?;
        Ok(())
    }

    fn clone_for_hyper(&self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(NyxoraNode {
            state: Arc::new(Mutex::new((*self.state.lock().unwrap()).clone())),
            consensus: Arc::new(Mutex::new((*self.consensus.lock().unwrap()).clone())),
        }))
    }

    fn get_status(&self) -> NodeState {
        self.state.lock().unwrap().clone()
    }

    fn stake_tokens(&self, amount: u64) -> bool {
        let state = self.state.lock().unwrap();
        let mut consensus = self.consensus.lock().unwrap();
        consensus.stake(&state.config.address, amount)
    }
}

async fn handle_request(
    req: Request<Body>,
    node: Arc<Mutex<NyxoraNode>>
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/status") => {
            let status = node.lock().unwrap().get_status();
            let json = serde_json::to_string(&status).unwrap();
            Ok(Response::new(Body::from(json)))
        },
        (&hyper::Method::POST, "/stake") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let amount: u64 = String::from_utf8_lossy(&body_bytes).parse().unwrap_or(0);

            let success = node.lock().unwrap().stake_tokens(amount);
            let response = if success {
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from("Stake successful"))
            } else {
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Stake failed"))
            };
            Ok(response.unwrap())
        },
        (&hyper::Method::GET, "/blocks") => {
            // Return current block height
            let height = node.lock().unwrap().get_status().block_height;
            let response = format!("Current block height: {}", height);
            Ok(Response::new(Body::from(response)))
        },
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration
    let config = if fs::metadata(&cli.config).is_ok() {
        let contents = fs::read_to_string(&cli.config)?;
        serde_json::from_str(&contents)?
    } else {
        eprintln!("Configuration file '{}' not found, using defaults", cli.config);
        NodeConfig::default()
    };

    // Update config based on CLI args
    let config = NodeConfig {
        is_validator: cli.validator,
        ..config
    };

    println!("Starting Nyxora node...");
    println!("Validator mode: {}", config.is_validator);
    println!("Quantum enabled: {}", config.quantum_enabled);

    // Create and start the node
    let node = NyxoraNode::new(config);

    // If running as validator, start block production
    if cli.validator {
        println!("Validator node started, listening for transactions...");
        // In a real implementation, we would start the block production loop here
    }

    // Start the HTTP server
    node.start_server(cli.port).await?;

    Ok(())
}