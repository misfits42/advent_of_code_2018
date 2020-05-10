use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

use std::collections::HashMap;
use std::collections::VecDeque;

/// Struct representing the structured licence file in AoC 2018 Day 8.println!
///
/// Wrapper for a graph storing node relationships and HashMap containing the metadata entries for
/// each node.
struct LicenceGraph {
    graph: DiGraphMap<i64, i64>,
    meta_records: HashMap<i64, Vec<i64>>,
}

impl LicenceGraph {
    /// Creates a new LicenceGraph with empty graph and metadata records.
    pub fn new() -> Self {
        LicenceGraph {
            graph: DiGraphMap::new(),
            meta_records: HashMap::new(),
        }
    }

    /// Calculates the sum of all metadata values contained within the licence graph.
    ///
    /// Used in Day 8 Part 1 solution.
    pub fn get_metadata_sum(&self) -> i64 {
        let mut result: i64 = 0;
        for entries in self.meta_records.values() {
            for entry in entries {
                result += entry;
            }
        }
        return result;
    }

    /// Calculates the total value of the root node.
    ///
    /// Used in Day 8 Part 2 solution.
    pub fn get_root_node_value(&self) -> i64 {
        // Root node has ID of 0
        return self.get_node_value(0);
    }

    /// Calculates the value for the specified node using the algorithm described in the Day 8
    /// Part 2 challenge.
    fn get_node_value(&self, node_id: i64) -> i64 {
        let num_children = self
            .graph
            .neighbors_directed(node_id, Direction::Outgoing)
            .count();
        if num_children == 0 {
            return self.meta_records.get(&node_id).unwrap().iter().sum();
        } else {
            // Find the IDs of all child nodes and sort to ensure ordering is correct.
            let mut child_ids = self
                .graph
                .neighbors_directed(node_id, Direction::Outgoing)
                .collect::<Vec<i64>>();
            child_ids.sort();
            // Calculate the metadata sum for all child nodes, as determined by node meta records.
            let mut child_meta_sum = 0;
            for meta in self.meta_records.get(&node_id).unwrap() {
                // Ignore metadata values of 0 - they do not correspond to a child node
                if *meta == 0 {
                    continue;
                }
                // Calculate child ID array index for the metadata value
                let index = (meta - 1) as usize;
                // Check if the index relates to one of the child nodes - otherwise ignore
                if index < child_ids.len() {
                    child_meta_sum += self.get_node_value(child_ids[index]);
                }
            }
            return child_meta_sum;
        }
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
    // Create empty licence graph
    let mut licence_graph = LicenceGraph::new();
    // Process the licence graph
    let mut last_id = -1;
    process_licence_nums(&mut licence_nums, &mut licence_graph, &mut last_id, -1);
    return licence_graph;
}

#[aoc(day8, part1)]
fn solve_part_1(input: &LicenceGraph) -> i64 {
    return input.get_metadata_sum();
}

#[aoc(day8, part2)]
fn solve_part_2(input: &LicenceGraph) -> i64 {
    return input.get_root_node_value();
}

/// Extracts nodes from the licence numbers sequence and adds the node and metadata entries to the
/// licence graph.
///
/// The ID numbers assigned to each node are monotonically increasing to ensure the ordering of the
/// child nodes is preserved.
///
/// Returns the ID of the last node added to the licence graph.
fn process_licence_nums(
    licence_nums: &mut VecDeque<i64>,
    licence_graph: &mut LicenceGraph,
    last_id: &mut i64,
    parent_id: i64,
) {
    // Increment last ID and set current ID
    *last_id += 1;
    let current_id = *last_id;
    // Get number child items and metadeta entries
    let num_child = licence_nums.pop_front().unwrap();
    let num_metadata = licence_nums.pop_front().unwrap();
    // Recursively extract each child
    for _i in 0..num_child {
        process_licence_nums(licence_nums, licence_graph, last_id, current_id);
    }
    // Get the metadata entries
    let mut metadata_entries: Vec<i64> = vec![];
    for _i in 0..num_metadata {
        metadata_entries.push(licence_nums.pop_front().unwrap());
    }
    // Add node and metadata entry to graph
    licence_graph
        .meta_records
        .insert(current_id, metadata_entries.to_vec());
    licence_graph.graph.add_node(current_id);
    // Add parent-child rel if we are not the root node
    if parent_id != -1 {
        licence_graph.graph.add_edge(parent_id, current_id, -1);
    }
}
