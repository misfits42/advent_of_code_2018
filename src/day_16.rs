use regex::Regex;
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

    pub fn get_instruction(&self) -> Vec<usize> {
        return self.instruction.clone();
    }

    pub fn get_reg_before(&self) -> Vec<usize> {
        return self.reg_before.clone();
    }
    
    pub fn get_reg_after(&self) -> Vec<usize> {
        return self.reg_after.clone();
    }
}

#[aoc_generator(day16)]
fn generate_input(input: &str) -> Vec<OpSample> {
    // Create iterator to read lines from input
    let mut lines = input.lines();
    let mut op_samples = Vec::<OpSample>::new();
    let register_regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    loop {
        let line = String::from(lines.next().unwrap());
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
        } else { // Break when we get to the sample program - for now.
            break;
        }
    }
    return op_samples;
}

#[derive(Copy, Clone, IntoEnumIterator)]
enum Operation {
    AddReg,
    AddImm,
    MulReg,
    MulImm,
    BitANDReg,
    BitANDImm,
    BitORReg,
    BitORImm,
    SetReg,
    SetImm,
    GtImmReg,
    GtRegImm,
    GtRegReg,
    EqImmReg,
    EqRegImm,
    EqRegReg
}

fn perform_operation(start: Vec<usize>, instruct: Vec<usize>, op: Operation) -> Vec<usize> {
    let mut output = start.clone();
    match op {
        Operation::AddReg => {
            let res = start[instruct[1]] + start[instruct[2]];
            output[instruct[3]] = res;
        },
        Operation::AddImm => {
            let res = start[instruct[1]] + instruct[2];
            output[instruct[3]] = res;
        },
        Operation::MulReg => {
            let res = start[instruct[1]] * start[instruct[2]];
            output[instruct[3]] = res;
        },
        Operation::MulImm => {
            let res = start[instruct[1]] * instruct[2];
            output[instruct[3]] = res;
        },
        Operation::BitANDReg => {
            let res = start[instruct[1]] & start[instruct[2]];
            output[instruct[3]] = res;
        },
        Operation::BitANDImm => {
            let res = start[instruct[1]] & instruct[2];
            output[instruct[3]] = res;
        },
        Operation::BitORReg => {
            let res = start[instruct[1]] | start[instruct[2]];
            output[instruct[3]] = res;
        },
        Operation::BitORImm => {
            let res = start[instruct[1]] | instruct[2];
            output[instruct[3]] = res;
        },
        Operation::SetReg => {
            output[instruct[3]] = start[instruct[1]];
        },
        Operation::SetImm => {
            output[instruct[3]] = instruct[1];
        },
        Operation::GtImmReg => {
            let comp = instruct[1] > start[instruct[2]];
            if comp {
                output[instruct[3]] = 1;
            } else {
                output[instruct[3]] = 0;
            }
        },
        Operation::GtRegImm => {
            let comp = start[instruct[1]] > instruct[2];
            if comp {
                output[instruct[3]] = 1;
            } else {
                output[instruct[3]] = 0;
            }
        },
        Operation::GtRegReg => {
            let comp = start[instruct[1]] > start[instruct[2]];
            if comp {
                output[instruct[3]] = 1;
            } else {
                output[instruct[3]] = 0;
            }
        },
        Operation::EqImmReg => {
            let comp = instruct[1] == start[instruct[2]];
            if comp {
                output[instruct[3]] = 1;
            } else {
                output[instruct[3]] = 0;
            }
        },
        Operation::EqRegImm => {
            let comp = start[instruct[1]] == instruct[2];
            if comp {
                output[instruct[3]] = 1;
            } else {
                output[instruct[3]] = 0;
            }
        },
        Operation::EqRegReg => {
            let comp = start[instruct[1]] == start[instruct[2]];
            if comp {
                output[instruct[3]] = 1;
            } else {
                output[instruct[3]] = 0;
            }
        }
    }
    return output;
}

#[aoc(day16, part1)]
fn solve_part_1(op_samples: &Vec<OpSample>) -> u64 {
    let mut total_count = 0;
    // Check each operation sample
    for samp in op_samples {
        let mut outmut_match_count = 0;
        let reg_after = samp.get_reg_after();
        // Try each operation
        for op in Operation::into_enum_iter() {
            // Perform operation and check if output matches output from operation sample
            let output = perform_operation(samp.get_reg_before(), samp.get_instruction(), op);
            let mut output_match = true;
            for i in 0..4 {
                if output[i] != reg_after[i] {
                    output_match = false;
                    break;
                }
            }
            if output_match {
                outmut_match_count += 1;
            }
            // Increment overall count if we find it matches at least 3 operations
            if outmut_match_count == 3 {
                total_count += 1;
                break;
            }
        }
    }
    return total_count;
}

#[aoc(day16, part2)]
fn solve_part_2(op_samples: &Vec<OpSample>) -> u64 {
    unimplemented!();
}

