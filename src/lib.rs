use petgraph::graph::UnGraph;

fn create_network() -> UnGraph<(), ()> {
    UnGraph::<(), ()>::from_edges(&[
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

pub fn show_network() {
    let network = create_network();

    println!("Num nodes: {}", network.node_count());
    println!("Num edges: {}", network.edge_count());
}
