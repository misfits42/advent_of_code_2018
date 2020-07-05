use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, IntoEnumIterator, Hash, PartialEq, Eq, Debug)]
pub enum Operation {
    AddReg,     // add register
    AddImm,     // add immediate
    MulReg,     // multiply register
    MulImm,     // multiply immediate
    BitANDReg,  // bitwise AND register
    BitANDImm,  // bitwise AND immediate
    BitORReg,   // bitwise OR register
    BitORImm,   // bitwise IR immediate
    SetReg,     // set register
    SetImm,     // set immediate
    GtImmReg,   // greater-than immediate/register
    GtRegImm,   // greater-than register/immediate
    GtRegReg,   // greater-than register/register
    EqImmReg,   // equal immediate/register
    EqRegImm,   // equal register/immediate
    EqRegReg    // equal register/register
}

impl Operation {
    pub fn from_string(s: &str) -> Option<Operation> {
        match s {
            "addr" => Some(Operation::AddReg),
            "addi" => Some(Operation::AddImm),
            "mulr" => Some(Operation::MulReg),
            "muli" => Some(Operation::MulImm),
            "banr" => Some(Operation::BitANDReg),
            "bani" => Some(Operation::BitANDImm),
            "borr" => Some(Operation::BitORReg),
            "bori" => Some(Operation::BitORImm),
            "setr" => Some(Operation::SetReg),
            "seti" => Some(Operation::SetImm),
            "gtir" => Some(Operation::GtImmReg),
            "gtri" => Some(Operation::GtRegImm),
            "gtrr" => Some(Operation::GtRegReg),
            "eqir" => Some(Operation::EqImmReg),
            "eqri" => Some(Operation::EqRegImm),
            "eqrr" => Some(Operation::EqRegReg),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Instruction {
    op: Operation,
    values: (usize, usize, usize)
}

impl Instruction {
    pub fn new(op: Operation, values: (usize, usize, usize)) -> Self {
        Self {
            op: op,
            values: values
        }
    }

    pub fn get_operation(&self) -> Operation {
        return self.op;
    }

    pub fn get_values(&self) -> (usize, usize, usize) {
        return self.values;
    }
}

pub struct WristComputer {
    registers: Vec<usize>,
    ip_reg: Option<usize>,
    ip_val: usize
}

impl WristComputer {
    pub fn new(ip_reg: Option<usize>) -> Self {
        Self {
            registers: vec![0; 6],
            ip_reg: ip_reg,
            ip_val: 0,
        }
    }

    pub fn update_register_zero(&mut self, val: usize) {
        self.registers[0] = val;
    }

    pub fn get_registers(&self) -> Vec<usize> {
        return self.registers.clone();
    }

    pub fn execute_program(&mut self, program: &Vec<Instruction>) {
        // Re-initialise the instruction pointer to 0
        self.ip_val = 0;

        if self.ip_reg.is_none() {
            for instruct in program {
                let after = WristComputer::perform_operation(&self.registers, &instruct);
                self.registers = after;
            }
        } else {
            loop {
                // Check if instruction pointer still within bounds of program
                if self.ip_val >= program.len() {
                    break;
                }
                // Write instruction pointer value to register
                self.registers[self.ip_reg.unwrap()] = self.ip_val;
                // Get next instruction to execute
                let instruction = program[self.ip_val];
                
                // println!("[{}] {:?} --- {:?}", self.ip_val, instruction.get_operation(), instruction.get_values());
                // println!(">>>> Reg state: {:?}", self.registers);
                // println!("");

                // Execute instruction
                let after = WristComputer::perform_operation(&self.registers, &instruction);
                self.registers = after;
                // Retrieve value from IP register and increment
                self.ip_val = self.registers[self.ip_reg.unwrap()] + 1;


            }
        }
    }

    pub fn perform_operation(before: &Vec<usize>, instruction: &Instruction) -> Vec<usize> {
        let op = instruction.get_operation();
        let values = instruction.get_values();
        let mut after = before.clone();
        match op {
            Operation::AddReg => {
                let res = before[values.0] + before[values.1];
                after[values.2] = res;
            },
            Operation::AddImm => {
                let res = before[values.0] + values.1;
                after[values.2] = res;
            },
            Operation::MulReg => {
                let res = before[values.0] * before[values.1];
                after[values.2] = res;
            },
            Operation::MulImm => {
                let res = before[values.0] * values.1;
                after[values.2] = res;
            },
            Operation::BitANDReg => {
                let res = before[values.0] & before[values.1];
                after[values.2] = res;
            },
            Operation::BitANDImm => {
                let res = before[values.0] & values.1;
                after[values.2] = res;
            },
            Operation::BitORReg => {
                let res = before[values.0] | before[values.1];
                after[values.2] = res;
            },
            Operation::BitORImm => {
                let res = before[values.0] | values.1;
                after[values.2] = res;
            },
            Operation::SetReg => {
                after[values.2] = before[values.0];
            },
            Operation::SetImm => {
                after[values.2] = values.0;
            },
            Operation::GtImmReg => {
                let comp = values.0 > before[values.1];
                if comp {
                    after[values.2] = 1;
                } else {
                    after[values.2] = 0;
                }
            },
            Operation::GtRegImm => {
                let comp = before[values.0] > values.1;
                if comp {
                    after[values.2] = 1;
                } else {
                    after[values.2] = 0;
                }
            },
            Operation::GtRegReg => {
                let comp = before[values.0] > before[values.1];
                if comp {
                    after[values.2] = 1;
                } else {
                    after[values.2] = 0;
                }
            },
            Operation::EqImmReg => {
                let comp = values.0 == before[values.1];
                if comp {
                    after[values.2] = 1;
                } else {
                    after[values.2] = 0;
                }
            },
            Operation::EqRegImm => {
                let comp = before[values.0] == values.1;
                if comp {
                    after[values.2] = 1;
                } else {
                    after[values.2] = 0;
                }
            },
            Operation::EqRegReg => {
                let comp = before[values.0] == before[values.1];
                if comp {
                    after[values.2] = 1;
                } else {
                    after[values.2] = 0;
                }
            }
        }
        return after;
    }
}