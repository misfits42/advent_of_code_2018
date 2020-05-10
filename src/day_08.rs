use petgraph::graphmap::DiGraphMap;

use std::collections::VecDeque;
use std::collections::HashMap;

use uuid::Uuid;

struct LicenceGraph {
    graph: DiGraphMap<Uuid, i64>,
    node_data: HashMap<Uuid, Vec<i64>>,
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
    let mut licence_nums = input.trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<VecDeque<i64>>();
    // Create empty licence graph
    let mut licence_graph = LicenceGraph::new();
    // Process the licence graph
    process_licence_nums(&mut licence_nums, &mut licence_graph, None);
    return licence_graph;
}

#[aoc(day8. part1)]
fn solve_part_1(input: &LicenceGraph) -> i64 {
    return input.get_metadata_sum();
}

fn process_licence_nums(licence_nums: &mut VecDeque<i64>, licence_graph: &mut LicenceGraph, parent_id: Option<Uuid>) {
    // Check if we have processed all licence details
    while !licence_nums.is_empty() {
        // Get number child items and metadeta entries
        let num_child = licence_nums.pop_front().unwrap();
        let num_metadata = licence_nums.pop_front().unwrap();
        let mut metadata_entries: Vec<i64> = vec![];
        // Get the metadata entries
        if num_child == 0 {
            for _i in 0..num_metadata {
                metadata_entries.push(licence_nums.pop_front().unwrap());
            }
        } else {
            for _i in 0..num_metadata {
                metadata_entries.push(licence_nums.pop_back().unwrap());
            }
            // Reverse to preserve order of metadata entries
            metadata_entries.reverse();
        }
        // Add metadata entry record
        let current_id = Uuid::new_v4();
        let res = licence_graph.node_data.insert(current_id, metadata_entries.to_vec());
        if res != None {
            println!("duplicate id detected");
        }
        licence_graph.graph.add_node(current_id);
        // Check if we are at the root node
        if parent_id != None {
            licence_graph.graph.add_edge(parent_id.unwrap(), current_id, -1);
        }
        // Check if we need to recurse to get child nodes
        if num_child > 0 {
            process_licence_nums(licence_nums, licence_graph, Some(current_id));
        }
    }
}
