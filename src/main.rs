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
        Instruction::Halt
    ];

    let gcd_program = vec![
        // Main Function
        Instruction::Write("\nEuclid's GCD".to_string()), // Output "Euclid's GCD"
        Instruction::Write("\nEnter a number:\t".to_string()), // Prompt for the first number
        Instruction::Read,              // Push the first input onto the stack (x)
        Instruction::Write("\nEnter a second number:\t".to_string()), // Prompt for the second number
        Instruction::Read,              // Push the second input onto the stack (y)

        // Call gcd(x, y)
        Instruction::Jump("gcd".to_string()), // Call the gcd function
        Instruction::Label("result".to_string()),
        Instruction::Write("\nRESULT:\t".to_string()), // Output "RESULT:"
        Instruction::Print,              // Print the result of gcd(x, y)
        Instruction::Write("\n".to_string()), // Output a newline
        Instruction::Halt,               // Stop execution

        // GCD Function
        Instruction::Label("gcd".to_string()), // Label for the gcd function
        Instruction::Dup,
        Instruction::Push(0),                 // Push 0 onto the stack
        Instruction::Eq,                      // Check if v == 0
        Instruction::Brt("gcd_return_u".to_string()), // If true, branch to gcd_return_u

            
        /*
        v
        u
        -- dup --
        v
        v
        u
        -- rot --
        u
        v
        v
        -- dup --
        u
        u
        v
        v
        -- rot --
        v
        u
        u
        v
        -- dup --
        v
        v
        u
        u
        v
        -- rot --
        u
        v
        v
        u
        v
        --swap--
        v
        u
        v
        u
        v
        -- div --
        (u/v)
        v
        u
        v
        -- mul --
        v * (u/v)
        u
        v
        -- sub --
        u - (u/v) * v
        v
        */


        // Else branch
        Instruction::Dup,
        Instruction::Rot,
        Instruction::Dup,
        Instruction::Rot,
        Instruction::Dup,
        Instruction::Rot,
        Instruction::Swap,
        Instruction::Div,
        Instruction::Mul,
        Instruction::Sub,
        Instruction::Jump("gcd".to_string()), // Recursive call gcd(v, u - (u/v) * v)

        // Return u branch
        Instruction::Label("gcd_return_u".to_string()), // Label for return u
        Instruction::Pop, // Remove v
        Instruction::Jump("result".to_string())
    ];


    machine.load_program(gcd_program);
    machine.execute();
}