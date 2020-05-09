use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
fn generate_input(input: &str) -> DiGraphMap<char, i64> {
    // Keep track of the steps added to graph
    let mut steps_graph: DiGraphMap<char, i64> = DiGraphMap::<char, i64>::new();
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
            if !steps_graph.contains_node(first_step) {
                steps_graph.add_node(first_step);
            }
            if !steps_graph.contains_node(second_step) {
                steps_graph.add_node(second_step);
            }
            steps_graph.add_edge(first_step, second_step, -1);
            // Expecting only only capture match per line, so break
            break;
        }
    }
    return steps_graph;
}

#[aoc(day7, part1)]
fn solve_part_1(input: &DiGraphMap<char, i64>) -> String {
    // Initialise variables to track queued steps and completed steps
    let total_steps = input.nodes().count();
    // Find steps with no parents - these are used to initialse the steps queue
    let mut in_counts = HashMap::<char, usize>::new();
    for node in input.nodes() {
        let in_count = input.neighbors_directed(node, Direction::Incoming).count();
        in_counts.insert(node, in_count);
    }
    in_counts.retain(|_k, v| *v == 0);
    // Initialise tracking vars
    let mut steps_queue = in_counts.keys().map(|x| *x).collect::<Vec<char>>();
    steps_queue.sort();
    let mut steps_order = String::new();
    let mut steps_completed = HashSet::<char>::new();
    // Process all steps
    loop {
        // Get current step
        let current_step = steps_queue[0];
        steps_queue.remove(0);
        // Conduct the current step
        steps_order.push(current_step);
        steps_completed.insert(current_step);
        // Stop if we have completed all steps
        if steps_completed.len() == total_steps {
            break;
        }
        // For each child node, check if it can be added to queue (have all its parents been done)
        for child in input.neighbors_directed(current_step, Direction::Outgoing) {
            let mut add_child_to_queue = true;
            for parent in input.neighbors_directed(child, Direction::Incoming) {
                if !steps_completed.contains(&parent) {
                    add_child_to_queue = false;
                    break;
                }
            }
            // Add child if all parents conducted, and re-sort the steps queue
            if add_child_to_queue {
                steps_queue.push(child);
                steps_queue.sort();
            }
        }
    }
    return steps_order;
}

#[aoc(day7, part2)]
fn solve_part_2(input: &DiGraphMap<char, i64>) -> u64 {
    // Initialise variables to track queued steps and completed steps
    let total_steps = input.nodes().count();
    // Find steps with no parents - these are used to initialse the steps queue
    let mut in_counts = HashMap::<char, usize>::new();
    for node in input.nodes() {
        let in_count = input.neighbors_directed(node, Direction::Incoming).count();
        in_counts.insert(node, in_count);
    }
    in_counts.retain(|_k, v| *v == 0);
    // Initialise tracking vars
    let mut steps_queue = in_counts.keys().map(|x| *x).collect::<Vec<char>>();
    steps_queue.sort();
    let mut steps_completed = HashSet::<char>::new();
    // Start with all 5 workers available
    let mut available_workers = 5;
    // Track each step in progress and seconds remaining until completion
    let mut steps_in_progress = HashMap::<char, u64>::new();
    // Track total time elapsed
    let mut seconds_elapsed = 0;
    loop {
        // Check if any steps have finished and process
        let mut steps_to_remove: Vec<char> = vec![];
        for (step, secs_rem) in steps_in_progress.iter() {
            if *secs_rem == 0 {
                steps_to_remove.push(*step);
            }
        }
        // Process steps that have completed
        for step in steps_to_remove {
            // Record step as completed and remove from in-progress listing
            steps_completed.insert(step);
            steps_in_progress.remove(&step);
            // Put the worker back in the available pool
            available_workers += 1;
            // Check if all steps have been processed - and return total time taken
            if steps_completed.len() == total_steps {
                return seconds_elapsed;
            }
            // For child node, check if it can be added to queue (have all its parents been done)
            for child in input.neighbors_directed(step, Direction::Outgoing) {
                let mut add_child_to_queue = true;
                for parent in input.neighbors_directed(child, Direction::Incoming) {
                    if !steps_completed.contains(&parent) {
                        add_child_to_queue = false;
                        break;
                    }
                }
                // Add child if all parents conducted, and re-sort the steps queue
                if add_child_to_queue {
                    steps_queue.push(child);
                    steps_queue.sort();
                }
            }
        }
        // Assign workers to steps
        while available_workers > 0 && !steps_queue.is_empty() {
            // Get next available step
            let next_step = steps_queue[0];
            steps_queue.remove(0);
            let step_duration = get_step_duration(next_step);
            // Take a worker from the available pool
            available_workers -= 1;
            // Add step to the progress listing
            steps_in_progress.insert(next_step, step_duration);
        }
        // Decrement time remaining steps in progress
        for time_remaining in steps_in_progress.values_mut() {
            *time_remaining -= 1;
        }
        // Increment time taken
        seconds_elapsed += 1;
    }
}

/// Calculates the duration of each step ('A' - 'Z').
/// 
/// Time taken is 60 seconds plus step-specified period ('A' is 1, 'B' is 2, and so on).
fn get_step_duration(step: char) -> u64 {
    return 60 + (step as u64 - ('A' as u64)) + 1;
}
