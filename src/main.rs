// Struct to represent Stack Machine
struct StackMachine {
    stack: Vec<i32>, // Use Vec, Rust's dynamic array
}

// Instruction enum to hold different instructions for the Stack Machine
enum Instruction {
    // Stack Operations
    Push(i32), Pop,
    // Dup, Swap, Rot, Clear,

    // Arithmetic
    Add, Sub, Mul, Div, Rem, Neg, Inc, Dec,

    // Comparison
    Eq, Ne, Le, Ge, Lt, Gt,

    // Control Flow
    Call, Ret, Retv, Jump, Brt, Brz, Halt, Label,

    // Memory
    Load, Save, Store, Alloc, Free,

    // I/O
    Print, Read, Write, Scan,

    // Bitwise/Logical
    And, Or, Xor, Not, Shl, Shr, Bool,

    // Debugging
    Dump, Trace
}

impl StackMachine {
    fn new() -> Self {
        StackMachine { stack: Vec::new() }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // Stack Operations
            Instruction::Push(value) => self.stack.push(value),
            Instruction::Pop => { self.stack.pop(); },
            
            // Arithmetic
            Instruction::Add => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a + b);
            },
            Instruction::Sub => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a - b);
            },
            Instruction::Mul => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a * b);
            },
            Instruction::Div => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a / b);
            },
            Instruction::Rem => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a); //TODO
            }
            Instruction::Neg => {
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(-a);
            }
            Instruction::Inc => {
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a + 1);
            }
            Instruction::Dec => {
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a - 1);
            }

            // I/O
            Instruction::Print => {
                if let Some(top) = self.stack.last() {
                    println!("{}", top);
                } else {
                    println!("Stack is empty");
                }
            },
        }
    }
}

fn main() {
    let mut machine = StackMachine::new();
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Add,
        Instruction::Print,
        Instruction::Push(20),
        Instruction::Sub,
        Instruction::Print,
    ];

    for instr in program {
        machine.execute(instr);
    }
}
