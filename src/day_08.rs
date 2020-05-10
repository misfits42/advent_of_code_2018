use petgraph::graphmap::DiGraphMap;

use std::collections::HashMap;
use std::collections::VecDeque;

struct LicenceGraph {
    graph: DiGraphMap<i64, i64>,
    node_data: HashMap<i64, Vec<i64>>,
}

impl LicenceGraph {
    pub fn new() -> Self {
        LicenceGraph {
            graph: DiGraphMap::new(),
            node_data: HashMap::new(),
        }
    }

    pub fn get_metadata_sum(&self) -> i64 {
        let mut result = 0;
        for entries in self.node_data.values() {
            for entry in entries {
                result += entry;
            }
        }
        return result;
    }
}

#[aoc_generator(day8)]
fn generate_input(input: &str) -> LicenceGraph {
    // Convert input into deque
    let mut licence_nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();
    //// println!("{:?}", licence_nums);
    // Create empty licence graph
    let mut licence_graph = LicenceGraph::new();
    // Process the licence graph
    process_licence_nums(&mut licence_nums, &mut licence_graph, -1);
    return licence_graph;
}

#[aoc(day8. part1)]
fn solve_part_1(input: &LicenceGraph) -> i64 {
    return input.get_metadata_sum();
}

/// Extracts nodes from the licence numbers sequence and adds the node and metadata entries to the
/// licence graph.
/// 
/// Returns the ID of the last node added to the licence graph.
fn process_licence_nums(
    licence_nums: &mut VecDeque<i64>,
    licence_graph: &mut LicenceGraph,
    parent_id: i64,
) -> i64 {
    let mut current_id = parent_id;
    // Get number child items and metadeta entries
    let num_child = licence_nums.pop_front().unwrap();
    let num_metadata = licence_nums.pop_front().unwrap();
    // Recursively extract each child
    for _i in 0..num_child {
        current_id = process_licence_nums(licence_nums, licence_graph, current_id);
    }
    // Get the metadata entries
    let mut metadata_entries: Vec<i64> = vec![];
    for _i in 0..num_metadata {
        metadata_entries.push(licence_nums.pop_front().unwrap());
    }
    // Add node and metadata entry to graph
    current_id += 1;
    licence_graph.node_data.insert(current_id, metadata_entries.to_vec());
    licence_graph.graph.add_node(current_id);
    // Add parent-child rel if we are not the root node
    if parent_id != -1 {
        licence_graph.graph.add_edge(parent_id, current_id, -1);
    }
    return current_id;
}
