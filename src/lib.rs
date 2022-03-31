use petgraph::algo;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

pub const NETWORK_SIZE: usize = 20;
pub const NUM_EDGES: usize = 28;
pub const PACKET_SIZE: u32 = 12_000;

pub fn create_network() -> StableGraph<(), (), Undirected> {
    StableGraph::<(), (), Undirected>::from_edges(&[
        (0, 1),
        (0, 3),
        (0, 6),
        (1, 2),
        (2, 4),
        (3, 8),
        (3, 16),
        (4, 5),
        (4, 10),
        (5, 13),
        (5, 15),
        (6, 9),
        (7, 9),
        (7, 10),
        (8, 11),
        (8, 14),
        (9, 12),
        (9, 13),
        (10, 12),
        (10, 18),
        (11, 17),
        (12, 16),
        (14, 15),
        (14, 17),
        (15, 19),
        (16, 17),
        (17, 18),
        (18, 19),
    ])
}

fn check_edges_capacity(
    capacities: [u32; NUM_EDGES],
    network: &StableGraph<(), (), Undirected>,
    flows: [u32; NUM_EDGES],
) -> bool {
    for edge in network.edge_indices() {
        let capacity = capacities[edge.index()];
        let flow = flows[edge.index()];

        if capacity / PACKET_SIZE < flow {
            return false;
        }
    }

    true
}

fn get_capacities() -> [u32; NUM_EDGES] {
    [
        100_000_000,
        1_000_000,
        10_000_000,
        10_000_000,
        100_000_000,
        1_000_000,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000_000,
        10_000_000,
        100_000_000,
        100_000_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000_000,
        10_000_000,
        100_000_000,
        100_000_000,
        10_000_000,
        1_000_000,
        10_000_000,
        100_000_000,
    ]
}

fn get_flows(
    intensity_matrix: [[u32; NETWORK_SIZE]; NETWORK_SIZE],
    network: &StableGraph<(), (), Undirected>,
) -> [u32; NUM_EDGES] {
    let mut flows = [0; 28];

    for i in 0..NETWORK_SIZE as u32 {
        for j in 0..NETWORK_SIZE as u32 {
            if i != j {
                let intensity = intensity_matrix[i as usize][j as usize];

                let (_, route) =
                    algo::astar(network, i.into(), |goal| goal == j.into(), |_| 1, |_| 0)
                        .expect("has to be road");

                let mut before_node_index = route[0];

                for node in &route[1..] {
                    let edge = network
                        .find_edge(before_node_index, *node)
                        .expect("has to be edge");

                    flows[edge.index()] += intensity;

                    before_node_index = *node;
                }
            }
        }
    }

    flows
}

fn create_intensity_matrix() -> [[u32; NETWORK_SIZE]; NETWORK_SIZE] {
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 8, 1, 2, 5, 9, 2, 1, 5],
        [5, 0, 2, 5, 1, 7, 4, 1, 9, 4, 1, 5, 9, 1, 2, 4, 3, 1, 2, 1],
        [7, 4, 0, 9, 4, 1, 5, 9, 1, 6, 5, 4, 3, 8, 1, 7, 4, 1, 9, 8],
        [9, 4, 1, 0, 9, 4, 5, 6, 7, 6, 5, 4, 5, 9, 1, 2, 4, 3, 2, 1],
        [1, 3, 8, 1, 0, 5, 8, 1, 5, 7, 6, 3, 3, 8, 7, 4, 4, 7, 7, 6],
        [5, 6, 1, 7, 3, 0, 7, 9, 5, 7, 4, 6, 4, 1, 9, 9, 4, 9, 4, 3],
        [8, 4, 1, 4, 2, 3, 0, 9, 7, 3, 1, 2, 5, 3, 8, 1, 4, 6, 9, 1],
        [6, 3, 3, 6, 1, 4, 4, 0, 2, 2, 9, 3, 7, 2, 9, 8, 6, 7, 1, 2],
        [4, 7, 3, 5, 3, 4, 6, 3, 0, 8, 3, 2, 8, 8, 1, 1, 1, 3, 9, 3],
        [8, 2, 8, 6, 1, 3, 4, 5, 3, 0, 5, 7, 7, 4, 5, 3, 1, 7, 5, 9],
        [3, 8, 2, 7, 1, 1, 4, 1, 2, 4, 0, 5, 3, 3, 1, 9, 6, 3, 9, 4],
        [3, 5, 9, 6, 9, 8, 7, 3, 9, 3, 5, 0, 2, 2, 4, 5, 6, 8, 2, 9],
        [3, 7, 8, 2, 2, 5, 5, 9, 2, 2, 3, 5, 0, 9, 4, 8, 4, 8, 2, 8],
        [2, 2, 5, 4, 4, 7, 4, 6, 2, 2, 6, 2, 8, 0, 6, 3, 5, 9, 6, 9],
        [2, 5, 8, 8, 1, 4, 7, 6, 3, 2, 5, 4, 2, 3, 0, 2, 7, 6, 1, 5],
        [2, 6, 8, 8, 3, 7, 1, 5, 5, 9, 5, 6, 6, 2, 1, 0, 2, 5, 7, 3],
        [4, 9, 3, 8, 5, 9, 2, 5, 6, 6, 6, 7, 1, 4, 8, 4, 0, 1, 6, 8],
        [7, 4, 6, 7, 6, 2, 8, 8, 6, 5, 4, 6, 2, 2, 6, 6, 4, 0, 7, 5],
        [3, 5, 4, 7, 6, 1, 6, 9, 5, 6, 1, 5, 1, 4, 7, 8, 9, 1, 0, 1],
        [7, 6, 3, 3, 5, 8, 8, 8, 5, 4, 1, 1, 5, 5, 9, 2, 8, 4, 2, 0],
    ]
}

fn show_network() {
    let network = create_network();

    let a = network.find_edge(0.into(), 1.into()).unwrap();
    println!("{:?}", a);

    println!("Num nodes: {}", network.node_count());
    println!("Num edges: {}", network.edge_count());
}

fn graph_connected(graph: &StableGraph<(), (), Undirected>) -> bool {
    let graph_size = graph.node_count();
    let node_map = algo::dijkstra(&graph, 0.into(), None, |_| 1);

    graph_size == node_map.len()
}
