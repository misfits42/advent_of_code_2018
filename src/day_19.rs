use regex::Regex;

use super::utils::wristcomp::WristComputer;
use super::utils::wristcomp::Instruction;
use super::utils::wristcomp::Operation;

#[aoc_generator(day19)]
fn generate_input(input: &str) -> (usize, Vec<Instruction>) {
    let ip_regex = Regex::new(r"#ip (\d)").unwrap();
    let instruct_regex = Regex::new(r"([a-z]+) (\d+) (\d+) (\d+)").unwrap();
    let mut lines = input.lines();
    let mut ip = usize::MAX;
    let mut program: Vec<Instruction> = vec![];
    while let Some(line) = lines.next() {
        if ip_regex.is_match(line) {
            // Line matching the instruction pointer regex will only match once
            for capture in ip_regex.captures_iter(line) {
                ip = capture[1].parse::<usize>().unwrap();
                break;
            }
        } else {
            for capture in instruct_regex.captures_iter(line) {
                let op = Operation::from_string(&capture[1]).unwrap();
                let val_a = capture[2].parse::<usize>().unwrap();
                let val_b = capture[3].parse::<usize>().unwrap();
                let val_c = capture[4].parse::<usize>().unwrap();
                let values = (val_a, val_b, val_c);
                let instruction = Instruction::new(op, values);
                program.push(instruction);
            }
        }
    }
    return (ip, program);
}

#[aoc(day19, part1)]
fn solve_part_1(input: &(usize, Vec<Instruction>)) -> usize {
    let mut wrist_computer = WristComputer::new(Some(input.0));
    wrist_computer.execute_program(&input.1);
    return wrist_computer.get_registers()[0];
}

#[aoc(day19, part2)]
fn solve_part_2(input: &(usize, Vec<Instruction>)) -> usize {
    let mut wrist_computer = WristComputer::new(Some(input.0));
    wrist_computer.update_register_zero(1);
    // Run background program time
    wrist_computer.execute_program(&input.1);
    return wrist_computer.get_registers()[0];
}
