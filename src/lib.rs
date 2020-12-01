use std::collections::VecDeque;

/// Representation of an Instruction to be executed by the Cpu
/// TODO: Add more instructions
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Max,
    Min,
    Push(StoredValue),
    Pop,
    PrintAccumulator,
    EOP, // End of program
}

/// Representation of a stored value in the stack
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StoredValue {
    Integer(i64),
    Float(f64),
    Character(char),
    Undefined,
    // TODO: Strings...
}

/// Stack type
pub type Stack = VecDeque<StoredValue>;

/// Reperesentation of the Computer unit that execs code
#[derive(Debug)]
pub struct Cpu {
    /// Program stored on the heap
    program: Vec<Instruction>,

    /// Stack containing the values used by the instuctions and the function call-satck (i think)
    stack: Stack,

    /// Program counter that tracks the current instruction to exec
    pc: usize,

    /// Accumulator register used to store the result of some stack operations, for efficiency
    accumulator: StoredValue,
}

impl Cpu {
    /// Function that creates a new Cpu from a instructions slice
    pub fn new(program_data: &[Instruction]) -> Option<Self> {
        let mut program = Vec::new();
        program.extend_from_slice(program_data);
        Some(Cpu {
            program,
            stack: Stack::with_capacity(4096 * 10), // 10MB
            pc: 0,
            accumulator: StoredValue::Undefined,
        })
    }
    
    /// Function that starts to execute Instructions
    pub fn run(&mut self) -> Option<()> {
        while let Some(instr) = self.fetch_instr() {
            // Match the instruction
            match instr {
                Instruction::Add => {
                    // Get the operands from the stack
                    let (a, b) = self.pop_stack_pair()?;
                    self.accumulator = match (a, b) {
                        (StoredValue::Integer(n1), StoredValue::Integer(n2)) 
                            => StoredValue::Integer(n1 + n2),
                        (StoredValue::Float(n1), StoredValue::Float(n2)) 
                            => StoredValue::Float(n1 + n2),
                        _ => panic!("The elements: {:?} and {:?} cannot be added", a, b),
                    };
                }
                Instruction::Pop => {
                    self.stack.pop_back()?;
                }
                Instruction::Push(v) => {
                    self.stack.push_back(v);
                }
                Instruction::PrintAccumulator => println!("Accumulator: {:?}", self.accumulator),
                Instruction::EOP => break, 
                _ => panic!("Unexpected instr: {:?}", instr),
            }
        }
        Some(())
    }
    
    /// Function that fetchs an instruction from `self.program` and increments the `self.pc`
    fn fetch_instr(&mut self) -> Option<Instruction> {
        match self.program.get(self.pc) {
            Some(&i) => {
                self.pc += 1;
                Some(i)
            },
            // I expect that the program ends with an specific Inturrupt
            None => panic!("Unexpected end of program!"),
        }
    }

    /// Function that retrieves from the stack two values
    fn pop_stack_pair(&mut self) -> Option<(StoredValue, StoredValue)> {
        let a = self.stack.pop_back()?;
        let b = self.stack.pop_back()?;
        Some((a, b))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_program_test() {
        let program = &[
            Instruction::Push(StoredValue::Integer(10)),
            Instruction::Push(StoredValue::Integer(1)),
            Instruction::Add,
            Instruction::PrintAccumulator,
            Instruction::EOP,
        ];
        let mut cpu = Cpu::new(program).unwrap();
        cpu.run().unwrap();
    }
}











