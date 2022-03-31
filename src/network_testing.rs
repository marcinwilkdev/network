use petgraph::algo;

use rand::prelude::*;

use crate::*;

pub fn test_network(
    network: &Network,
    intensity_matrix: NodesMatrix,
    capacities: &[u32],
    p: f64,
    t_max: f64,
    routes_cache: &RoutesCache,
) -> f64 {
    let mut s = 0.0;

    for _ in 0..TEST_COUNT {
        let mut network_copy = network.clone();

        disconnect_faulty_connections(&mut network_copy, p);

        // let multiplier = TEST_COUNT / 100;

        // if i % multiplier == 0 {
        //     println!("{}% done", i / multiplier);
        // }

        if !is_network_connected(&network_copy) {
            continue;
        }

        let flows = get_flows(intensity_matrix, &network_copy, routes_cache, capacities.len());

        if !check_edges_capacity(capacities, &network_copy, &flows) {
            continue;
        }

        let t = calculate_delay(intensity_matrix, &network_copy, capacities, &flows);

        if t < t_max {
            s += 1.0;
        }
    }

    s / TEST_COUNT as f64
}

fn disconnect_faulty_connections(network: &mut Network, p: f64) {
    let mut edge_indices = Vec::new();

    for edge_indice in network.edge_indices() {
        let r = thread_rng().gen::<f64>();

        if r > p {
            edge_indices.push(edge_indice);
        }
    }

    for edge_indice in edge_indices {
        network.remove_edge(edge_indice);
    }
}

fn is_network_connected(network: &Network) -> bool {
    let network_size = network.node_count();
    let node_map = algo::dijkstra(network, 0.into(), None, |_| 1);

    network_size == node_map.len()
}

pub fn generate_routes_cache(network: &Network) -> RoutesCache {
    let mut routes_cache = RoutesCache::new();

    for i in 0..NETWORK_SIZE as u32 {
        for j in 0..NETWORK_SIZE as u32 {
            if i != j {
                let (_, route) =
                    algo::astar(network, i.into(), |goal| goal == j.into(), |_| 1, |_| 0)
                        .expect("has to be road");

                let mut route_edges = Vec::new();

                let mut before_node_index = route[0];

                for node in &route[1..] {
                    let edge = network
                        .find_edge(before_node_index, *node)
                        .expect("has to be edge");

                    route_edges.push(edge);

                    before_node_index = *node;
                }

                routes_cache.insert((i, j), route_edges);
            }
        }
    }

    routes_cache
}

fn check_cached_route_is_valid(
    network: &Network,
    cached_route: &[petgraph::graph::EdgeIndex],
) -> bool {
    for edge in cached_route {
        if network.edge_weight(*edge).is_none() {
            return false;
        }
    }

    true
}

fn get_flows(
    intensity_matrix: NodesMatrix,
    network: &Network,
    routes_cache: &RoutesCache,
    num_edges: usize,
) -> Vec<u32> {
    let mut flows = vec![0; num_edges];

    for i in 0..NETWORK_SIZE as u32 {
        for j in 0..NETWORK_SIZE as u32 {
            if i != j {
                let cached_route = routes_cache.get(&(i, j)).expect("has to be cached route");
                let intensity = intensity_matrix[i as usize][j as usize];

                if check_cached_route_is_valid(network, cached_route) {
                    for edge in cached_route {
                        flows[edge.index()] += intensity;
                    }
                } else {
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
    }

    flows
}

fn check_edges_capacity(capacities: &[u32], network: &Network, flows: &[u32]) -> bool {
    for edge in network.edge_indices() {
        let capacity = capacities[edge.index()];
        let flow = flows[edge.index()];

        if capacity / PACKET_SIZE < flow {
            return false;
        }
    }

    true
}

fn calculate_delay(
    intensity_matrix: NodesMatrix,
    network: &Network,
    capacities: &[u32],
    flows: &[u32],
) -> f64 {
    let sum_intensities = intensity_matrix
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum::<u32>();

    let inverse_sum_intensities = 1.0 / sum_intensities as f64;

    inverse_sum_intensities
        * network
            .edge_indices()
            .map(|indice| {
                let indice = indice.index();
                let flow = flows[indice] as f64;
                let capacity = capacities[indice] as f64;

                flow / ((capacity / PACKET_SIZE as f64) - flow)
            })
            .sum::<f64>()
}

pub fn add_edge(network: &mut Network) {
    loop {
        let first_node_indice = thread_rng().gen_range(0..NETWORK_SIZE as u32);
        let second_node_indice = thread_rng().gen_range(0..NETWORK_SIZE as u32);

        if first_node_indice == second_node_indice {
            continue;
        }

        let edge = network.find_edge(first_node_indice.into(), second_node_indice.into());

        if edge.is_some() {
            continue;
        }

        network.add_edge(first_node_indice.into(), second_node_indice.into(), ());

        break;
    }
}
