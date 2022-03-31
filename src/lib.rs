pub mod data_generators;
pub mod network_testing;

pub use network_testing::{test_network, generate_routes_cache, add_edge};
pub use data_generators::{generate_network, generate_intensity_matrix, generate_capacities};

pub const NETWORK_SIZE: usize = 20;
pub const NUM_EDGES: usize = 28;
pub const PACKET_SIZE: u32 = 120;
pub const TEST_COUNT: usize = 1_000_000;

pub type Network = petgraph::stable_graph::StableGraph<(), (), petgraph::Undirected>;
pub type NodesMatrix = [[u32; NETWORK_SIZE]; NETWORK_SIZE];
pub type RoutesCache = std::collections::HashMap<(u32, u32), Vec<petgraph::graph::EdgeIndex>>;
