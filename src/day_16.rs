use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

use super::utils::wristcomp::Instruction;
use super::utils::wristcomp::Operation;
use super::utils::wristcomp::WristComputer;

use enum_iterator::IntoEnumIterator;

struct OpSample {
    reg_before: Vec<usize>,
    reg_after: Vec<usize>,
    instruction: Vec<usize>
}

impl OpSample {
    pub fn new(reg_before: Vec<usize>, reg_after: Vec<usize>, instruction: Vec<usize>) -> Self {
        Self {
            reg_before: reg_before,
            reg_after: reg_after,
            instruction: instruction
        }
    }

    pub fn get_opcode(&self) -> usize {
        return self.instruction[0];
    }

    pub fn get_reg_before(&self) -> Vec<usize> {
        return self.reg_before.clone();
    }
    
    pub fn get_reg_after(&self) -> Vec<usize> {
        return self.reg_after.clone();
    }

    pub fn get_values(&self) -> (usize, usize, usize) {
        return (self.instruction[1], self.instruction[2], self.instruction[3]);
    }
}

#[aoc_generator(day16)]
fn generate_input(input: &str) -> (Vec<OpSample>, Vec<Vec<usize>>) {
    // Create iterator to read lines from input
    let mut lines = input.lines();
    let mut op_samples = Vec::<OpSample>::new();
    let mut test_program = Vec::<Vec<usize>>::new();
    let register_regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    loop {
        let next = lines.next();
        if next.is_none() {
            break;
        }
        let line = String::from(next.unwrap());
        if line.is_empty() {
            continue;
        } else if line.starts_with("Before") {
            // Extract register values before applying instruction
            let mut reg_before = Vec::<usize>::new();
            for capture in register_regex.captures_iter(&line) {
                // Regex will only match once
                for i in 1..=4 {
                    reg_before.push(capture[i].parse::<usize>().unwrap());
                }
                break;
            }
            let line = String::from(lines.next().unwrap());
            // Extract instruction values
            let instruct = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let line = String::from(lines.next().unwrap());
            // Extract register values after applying instruction
            let mut reg_after = Vec::<usize>::new();
            for capture in register_regex.captures_iter(&line) {
                // Regex will only match once
                for i in 1..=4 {
                    reg_after.push(capture[i].parse::<usize>().unwrap());
                }
                break;
            }
            // Create new OpSample from extracted values
            let op_sample = OpSample::new(reg_before, reg_after, instruct);
            op_samples.push(op_sample);
        } else { // Get instruction line for test program
            let instruct = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            test_program.push(instruct);
        }
    }
    return (op_samples, test_program);
}


#[aoc(day16, part1)]
fn solve_part_1(input: &(Vec<OpSample>, Vec<Vec<usize>>)) -> u64 {
    let mut total_count = 0;
    // Check each operation sample
    for samp in &input.0 {
        let mut output_match_count = 0;
        let reg_before = samp.get_reg_before();
        let reg_after = samp.get_reg_after();
        let values = samp.get_values();
        // Try each operation
        for op in Operation::into_enum_iter() {
            let instruction = Instruction::new(op, values);
            // Perform operation and check if output matches output from operation sample
            let op_output = WristComputer::perform_operation(&reg_before, &instruction);
            let output_match = check_vectors_equal_elements(&reg_after, &op_output);
            if output_match {
                output_match_count += 1;
            }
            // Increment overall count if we find it matches at least 3 operations
            if output_match_count == 3 {
                total_count += 1;
                break;
            }
        }
    }
    return total_count;
}

#[aoc(day16, part2)]
fn solve_part_2(input: &(Vec<OpSample>, Vec<Vec<usize>>)) -> usize {
    // Determine mapping of opcodes to operations
    let mut opcode_poss = HashMap::<usize, HashSet<Operation>>::new();
    for i in 0..16 {
        opcode_poss.insert(i, HashSet::<Operation>::new());
    }
    // Process each operation sample
    for samp in &input.0 {
        let reg_before = samp.get_reg_before();
        let reg_after = samp.get_reg_after();
        let values = samp.get_values();
        for op in Operation::into_enum_iter() {
            let instruction = Instruction::new(op, values);
            // Perform operation and check if output matches sample output
            let op_output = WristComputer::perform_operation(&reg_before, &instruction);
            let output_match = check_vectors_equal_elements(&reg_after, &op_output);
            // If output matches, add the operation as a possible mapping for the opcode
            if output_match {
                opcode_poss.get_mut(&samp.get_opcode()).unwrap().insert(op);
            }
        }
    }
    // Determine opcode to operation mapping by reducing possibilities using unique mappings
    let opcode_mapping = determine_opcode_mappings(opcode_poss);
    // Generate the test program
    let mut program: Vec<Instruction> = vec![];
    for instruct in &input.1 {
        let opcode = instruct[0];
        let op = *opcode_mapping.get(&opcode).unwrap();
        let instruction = Instruction::new(op, (instruct[1], instruct[2], instruct[3]));
        program.push(instruction);
    }
    // Execute the test program
    let mut wrist_comp = WristComputer::new(None);
    wrist_comp.execute_program(program);
    // Return the value in register 0 after executing test program
    return wrist_comp.get_registers()[0];
}

fn check_vectors_equal_elements<T: PartialEq>(left: &Vec<T>, right: &Vec<T>) -> bool {
    // First check if vectors are the same length
    if left.len() != right.len() {
        return false
    }
    for i in 0..left.len() {
        if left[i] != right[i] {
            return false;
        }
    }
    return true;
}

fn determine_opcode_mappings(opcode_poss: HashMap<usize, HashSet<Operation>>) -> HashMap<usize, Operation> {
    let mut opcode_mapping = HashMap::<usize, Operation>::new();
    let mut opcode_counts = opcode_poss.clone();
    loop {
        // Check which opcodes have only one possible operation
        let mut uniq_opcodes = Vec::<usize>::new();
        for (opcode, possibles) in opcode_counts.iter() {
            if possibles.len() == 1 {
                uniq_opcodes.push(*opcode);
            }
        }
        // Add mappings for uniq opcodes
        for opcode in uniq_opcodes {
            // Get the operation mapped to the opcode
            let op = *opcode_counts.get(&opcode).unwrap().iter().next().unwrap();
            opcode_mapping.insert(opcode, op);
            // Remove the opcode from the opcode count
            opcode_counts.remove(&opcode);
        }
        // Check if we have any move opcodes left to map
        if opcode_counts.len() == 0 {
            break;
        }
        // Remove unique opcodes from possibles in opcode counts
        for (_opcode, possibles) in opcode_counts.iter_mut() {
            for op in opcode_mapping.values() {
                possibles.remove(op);
            }
        }
    }
    return opcode_mapping;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d16_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day16.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(531, result);
    }

    #[test]
    fn test_d16_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day16.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(649, result);
    }
}
