mod stack_machine;
use stack_machine::{StackMachine, Instruction};

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
