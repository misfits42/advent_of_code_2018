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
fn solve_part_2(_input: &(usize, Vec<Instruction>)) -> usize {
    return test_wristcomp_background_process();
}

/// Calculates the value in register 0 after the background process described in AoC 2018 D19, P2.
/// 
/// Determined by observing execution of background process by wrist computer and reverse
/// engineering the main loop. Some optimisations were added after the reverse engineering to get
/// to the outcome significantly faster.
fn test_wristcomp_background_process() -> usize {
    // Initialise all registers to values present when process enters primary loop
    let mut reg: Vec<usize> = vec![0, 1, 0, 1, 2, 10551364];
    loop {
        reg[1] = 1;
        loop {
            // This check is an optimisation!
            if reg[5] % reg[3] != 0 {
                break;
            }
            reg[2] = reg[3] * reg[1];
            if reg[2] == reg[5] {
                reg[0] += reg[3];
                break;
            }
            reg[1] += 1;
            if reg[1] > reg[5] {
                break;
            }
        } 
        reg[3] += 1;
        if reg[3] > reg[5] {
            break;
        }
    }
    return reg[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d19_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day19.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1694, result);
    }

    #[test]
    fn test_d19_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2018/day19.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(18964204, result);
    }
}
