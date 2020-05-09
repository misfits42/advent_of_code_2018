use petgraph::Graph;
use petgraph::prelude::NodeIndex;

use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn generate_input(input: &str) -> Graph<char, i64> {
    // Keep track of the steps added to graph
    let mut steps: HashMap<char, NodeIndex> = HashMap::new();
    let mut steps_graph: Graph<char, i64> = Graph::<char, i64>::new();
    // Keep track of required step relationships
    let mut step_rels: Vec<(char, char)> = vec![];
    // Create regex to extraxt steps and relationships
    let line_regex = Regex::new(r"Step ([A-Z]) (.*?) ([A-Z]) can begin.").unwrap();
    // Process each line
    for line in input.lines() {
        let line = line.trim();
        for capture in line_regex.captures_iter(line) {
            // Extract the steps from the capture
            let first_step = capture[1].chars().collect::<Vec<char>>()[0];
            let second_step = capture[3].chars().collect::<Vec<char>>()[0];
            // Add steps to graph if not already
            if !steps.contains_key(&first_step) {
                let first_step_node = steps_graph.add_node(first_step);
                steps.insert(first_step, first_step_node);
            }
            if !steps.contains_key(&second_step) {
                let second_step_node = steps_graph.add_node(second_step);
                steps.insert(second_step, second_step_node);
            }
            // Record step relationship
            step_rels.push((first_step, second_step));
            // Expecting only only capture match per line, so break
            break;
        }
    }
    // Set up all step relationships
    for rel in step_rels {
        let first_step_node = *steps.get(&rel.0).unwrap();
        let second_step_node = *steps.get(&rel.1).unwrap();
        steps_graph.add_edge(first_step_node, second_step_node, -1);
    }
    return steps_graph;
}

#[aoc(day7, part1)]
fn solve_part_1(input: &Graph<char, i64>) -> String {
    println!("{:?}", input);
    return String::new();
}

