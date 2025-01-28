use std::{collections::HashMap, vec};

// Struct to represent Stack Machine
struct StackMachine {
    stack: Vec<i32>, // The stack holds integers for the machine's operations
    memory: HashMap<i32, i32>, // A simple key-value store for memory operations
    labels: HashMap<String, usize>, // Maps labels to their positions in the program
    program_counter: usize, // Tracks the current instruction to execute
    program: Vec<Instruction>, // The list of instructions to execute
}

// Instruction enum to hold different instructions for the Stack Machine
#[derive(Clone)]
enum Instruction {
    // Stack Operations
    Push(i32), Pop, Dup, Swap, Rot, Clear,

    // Arithmetic
    Add, Sub, Mul, Div, Rem, Neg, Inc, Dec,

    // Comparison
    Eq, Ne, Le, Ge, Lt, Gt,

    // Control Flow
    Call(String), Ret, Retv(i32), Jump(String), Brt(String), Brz(String), Halt, Label(String),

    // Memory
    Load(i32), Store(i32), Alloc(i32), Free(i32),

    // I/O
    Print, Read, Write(String), Scan,

    // Bitwise/Logical
    And, Or, Xor, Not, Shl, Shr, Bool,

    // Debugging
    Dump, Trace(bool),
}

impl StackMachine {
    fn new() -> Self {
        StackMachine {
            stack: Vec::new(),
            memory: HashMap::new(),
            labels: HashMap::new(),
            program_counter: 0,
            program: Vec::new(),
        }
    }

    fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.index_labels(); // Precompute label positions for faster jumps
    }

    fn index_labels(&mut self) {
        for (index, instruction) in self.program.iter().enumerate() {
            if let Instruction::Label(name) = instruction {
                self.labels.insert(name.clone(), index); // Map label names to their positions
            }
        }
    }

    fn execute(&mut self) {
        while self.program_counter < self.program.len() {
            let instruction = self.program[self.program_counter].clone(); // Fetch the next instruction
            self.program_counter += 1; // Move to the next instruction

            match instruction {
                // Stack Operations
                Instruction::Push(value) => self.stack.push(value), // Push a value onto the stack
                Instruction::Pop => { self.stack.pop(); }, // Remove the top value from the stack
                Instruction::Dup => {
                    if let Some(&top) = self.stack.last() {
                        self.stack.push(top); // Duplicate the top value of the stack
                    }
                },
                Instruction::Swap => {
                    if self.stack.len() >= 2 {
                        let len = self.stack.len();
                        self.stack.swap(len - 1, len - 2); // Swap the top two values on the stack
                    }
                },
                Instruction::Rot => {
                    if self.stack.len() >= 3 {
                        let a = self.stack.pop().expect("Stack underflow"); // 1st element
                        let b = self.stack.pop().expect("Stack underflow"); // 2nd element
                        let c = self.stack.pop().expect("Stack underflow"); // 3rd element
                        self.stack.push(a);
                        self.stack.push(c);
                        self.stack.push(b);
                    }
                },
                Instruction::Clear => self.stack.clear(), // Clear the entire stack

                // Arithmetic
                Instruction::Add => self.binary_op(|a, b| a + b),
                Instruction::Sub => self.binary_op(|a, b| a - b),
                Instruction::Mul => self.binary_op(|a, b| a * b),
                Instruction::Div => self.binary_op(|a, b| a / b),
                Instruction::Rem => self.binary_op(|a, b| a % b),
                Instruction::Neg => self.unary_op(|a| -a),
                Instruction::Inc => self.unary_op(|a| a + 1),
                Instruction::Dec => self.unary_op(|a| a - 1),

                // Comparison
                Instruction::Eq => self.binary_op(|a, b| if a == b { 1 } else { 0 }),
                Instruction::Ne => self.binary_op(|a, b| if a != b { 1 } else { 0 }),
                Instruction::Le => self.binary_op(|a, b| if a <= b { 1 } else { 0 }),
                Instruction::Ge => self.binary_op(|a, b| if a >= b { 1 } else { 0 }),
                Instruction::Lt => self.binary_op(|a, b| if a < b { 1 } else { 0 }),
                Instruction::Gt => self.binary_op(|a, b| if a > b { 1 } else { 0 }),

                // Control Flow
                Instruction::Jump(label) => {
                    if let Some(&pos) = self.labels.get(&label) {
                        self.program_counter = pos; // Jump to the label's position
                    }
                },
                Instruction::Brt(label) => {
                    if self.stack.pop().unwrap_or(0) != 0 {
                        if let Some(&pos) = self.labels.get(&label) {
                            self.program_counter = pos; // Branch if the top value is nonzero
                        }
                    }
                },
                Instruction::Brz(label) => {
                    if self.stack.pop().unwrap_or(0) == 0 {
                        if let Some(&pos) = self.labels.get(&label) {
                            self.program_counter = pos; // Branch if the top value is zero
                        }
                    }
                },
                Instruction::Halt => break, // Stop execution

                // Memory
                Instruction::Load(address) => {
                    let value = self.memory.get(&address).copied().unwrap_or(0);
                    self.stack.push(value); // Push the value at the given address
                },
                Instruction::Store(address) => {
                    if let Some(value) = self.stack.pop() {
                        self.memory.insert(address, value); // Store the top value at the given address
                    }
                },
                Instruction::Alloc(size) => {
                    for i in 0..size {
                        self.memory.insert(i, 0); // Allocate memory initialized to zero
                    }
                },
                Instruction::Free(address) => {
                    self.memory.remove(&address); // Free memory at the given address
                },

                // I/O
                Instruction::Print => {
                    if let Some(&top) = self.stack.last() {
                        println!("{}", top); // Print the top value of the stack
                    } else {
                        println!("Stack is empty");
                    }
                },
                Instruction::Read => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read input");
                    if let Ok(value) = input.trim().parse::<i32>() {
                        self.stack.push(value); // Read an integer and push it onto the stack
                    }
                },
                Instruction::Write(message) => {
                    println!("{}", message); // Print the given message
                },
                Instruction::Scan => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read input");
                    println!("Scanned: {}", input.trim()); // Read a line and display it
                },

                // Bitwise/Logical
                Instruction::And => self.binary_op(|a, b| a & b),
                Instruction::Or => self.binary_op(|a, b| a | b),
                Instruction::Xor => self.binary_op(|a, b| a ^ b),
                Instruction::Not => self.unary_op(|a| !a),
                Instruction::Shl => self.binary_op(|a, b| a << b),
                Instruction::Shr => self.binary_op(|a, b| a >> b),
                Instruction::Bool => self.unary_op(|a| if a != 0 { 1 } else { 0 }),

                // Debugging
                Instruction::Dump => {
                    println!("Stack: {:?}", self.stack); // Print the current stack
                    println!("Memory: {:?}", self.memory); // Print the current memory state
                },
                Instruction::Trace(on) => {
                    if on {
                        println!("Tracing enabled");
                    } else {
                        println!("Tracing disabled");
                    }
                },

                _ => (),
            }
        }
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let b = self.stack.pop().expect("Stack underflow"); // Pop the second operand
        let a = self.stack.pop().expect("Stack underflow"); // Pop the first operand
        self.stack.push(op(a, b)); // Apply the operation and push the result
    }

    fn unary_op<F>(&mut self, op: F)
    where
        F: Fn(i32) -> i32,
    {
        let a = self.stack.pop().expect("Stack underflow"); // Pop the operand
        self.stack.push(op(a)); // Apply the operation and push the result
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
        Instruction::Halt,
    ];

    let gcd_program = vec![
        // Main Function
        Instruction::Write("\nEuclid's GCD".to_string()), // Output "Euclid's GCD"
        Instruction::Write("\nEnter a number:\t".to_string()), // Prompt for the first number
        Instruction::Scan,              // Read input
        Instruction::Read,              // Push the first input onto the stack (x)
        Instruction::Write("\nEnter a second number:\t".to_string()), // Prompt for the second number
        Instruction::Scan,              // Read input
        Instruction::Read,              // Push the second input onto the stack (y)

        // Call gcd(x, y)
        Instruction::Call("gcd".to_string()), // Call the gcd function
        Instruction::Write("\nRESULT:\t".to_string()), // Output "RESULT:"
        Instruction::Print,              // Print the result of gcd(x, y)
        Instruction::Write("\n".to_string()), // Output a newline
        Instruction::Halt,               // Stop execution

        // GCD Function
        Instruction::Label("gcd".to_string()), // Label for the gcd function
        Instruction::Dup,                     // Duplicate the top value (v)
        Instruction::Push(0),                 // Push 0 onto the stack
        Instruction::Eq,                      // Check if v == 0
        Instruction::Brt("gcd_return_u".to_string()), // If true, branch to gcd_return_u

        // Else branch
        Instruction::Dup,                     // Duplicate v
        Instruction::Rot,                     // Bring u below v to the top
        Instruction::Dup,                     // Duplicate u
        Instruction::Dup,                     // Duplicate u
        Instruction::Swap,                    // Swap the top two (u and v)
        Instruction::Div,                     // Compute u/v
        Instruction::Mul,                     // Compute (u/v) * v
        Instruction::Sub,                     // Compute u - (u/v) * v
        Instruction::Rot,                     // Rotate stack (v becomes the top)
        Instruction::Call("gcd".to_string()), // Recursive call gcd(v, u - (u/v) * v)
        Instruction::Ret,                     // Return from function

        // Return u branch
        Instruction::Label("gcd_return_u".to_string()), // Label for return u
        Instruction::Swap,                    // Bring u to the top
        Instruction::Pop,                     // Remove v
        Instruction::Ret,                     // Return from function
    ];

    machine.load_program(gcd_program);
    machine.execute();
}
