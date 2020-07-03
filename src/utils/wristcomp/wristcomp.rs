use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, IntoEnumIterator, Hash, PartialEq, Eq, Debug)]
pub enum Operation {
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
    ip: Option<usize>
}

impl WristComputer {
    pub fn new(ip: Option<usize>) -> Self {
        Self {
            registers: vec![0; 6],
            ip: ip
        }
    }

    pub fn get_registers(&self) -> Vec<usize> {
        return self.registers.clone();
    }

    pub fn execute_program(&mut self, program: Vec<Instruction>) {
        if self.ip.is_none() {
            for instruct in program {
                let after = WristComputer::perform_operation(&self.registers, &instruct);
                self.registers = after;
            }
        } else {
            unimplemented!();
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