// Struct to represent Stack Machine
struct StackMachine {
    stack: Vec<i32>, // Use Vec, Rust's dynamic array
}

// Instruction enum to hold different instructions for the Stack Machine
enum Instruction {
    // Stack Operations

    Push(i32),   // Push a value onto the stack
    Pop,         // Remove the top value
    // Dup,
    // Swap,
    // Rot,
    // Clear,

    // Arithmetic
    Add,         // Add the top two values
    Sub,         // Subtract the top two values
    // Mul,
    // Div,
    // Rem,
    // Neg,
    // Inc,
    // Dec,

    // Comparison
    // Eq,
    // Ne,
    // Le,
    // Ge,
    // Lt,
    // Gt,

    // I/O
    Print,       // Print the top value
}

impl StackMachine {
    fn new() -> Self {
        StackMachine { stack: Vec::new() }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // Stack Operations
            Instruction::Push(value) => self.stack.push(value),
            Instruction::Pop => { self.stack.pop(); }, // Remove the top element

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
