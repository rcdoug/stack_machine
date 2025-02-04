use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write, Result};
use std::time::Duration;

mod stack_machine;
use stack_machine::{Instruction, StackMachine};

fn main() -> Result<()> {
    // Set up the terminal: enable raw mode and switch to the alternate screen.
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    // Create a sample program.
    // This program will:
    //   1. Push 10 and 20, add them, then print (should display 30).
    //   2. Push 5, multiply (30*5 = 150), then print.
    //   3. Request user input (a number), then push 2, multiply, and print the result.
    //   4. Halt.
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Add,
        Instruction::Print,      // Expected output: 30
        Instruction::Push(5),
        Instruction::Mul,
        Instruction::Print,      // Expected output: 150
        Instruction::Write("Enter a number:".to_string()),
        Instruction::Read,       // Will prompt: "Enter a number:"
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Print, 
        Instruction::Print,
        Instruction::Print,     // Prints result after multiplying user input by 2
        Instruction::Halt,
    ];

    let mut machine = StackMachine::new();
    machine.load_program(program);

    // Buffer to collect characters when the machine is waiting for input.
    let mut input_buffer = String::new();

    loop {
        // Clear the screen.
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;

        // Get terminal dimensions.
        let (cols, rows) = terminal::size()?;
        let left_panel_width = cols / 2;

        // --- Left Panel: Program Instructions ---
        execute!(stdout, cursor::MoveTo(0, 0), Print("Program Instructions:\n"))?;
        for (i, instr) in machine.get_program().iter().enumerate() {
            if i == machine.get_program_counter() {
                // Highlight the current instruction.
                execute!(stdout, SetForegroundColor(Color::Yellow))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::White))?;
            }
            execute!(stdout, Print(format!("{:>3}: {:?}\n", i, instr)))?;
        }
        execute!(stdout, ResetColor)?;

        // --- Right Panel: Stack Contents ---
        execute!(stdout, cursor::MoveTo(left_panel_width + 2, 0), Print("Stack Contents:\n"))?;
        for (i, value) in machine.get_stack().iter().enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(left_panel_width + 2, (i + 1) as u16),
                Print(format!("{}", value))
            )?;
        }

        // --- Bottom Panel: Machine Output and (if applicable) Input Prompt ---
        let output_panel_height = 7;
        let output_panel_start = rows.saturating_sub(output_panel_height);
        execute!(stdout, cursor::MoveTo(0, output_panel_start), Print("Machine Output:"))?;
        // Display only the last few output lines.
        let start_line = if machine.output.len() > (output_panel_height as usize - 2) {
            machine.output.len() - (output_panel_height as usize - 2)
        } else {
            0
        };
        for (i, line) in machine.output.iter().skip(start_line).enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(0, output_panel_start + 1 + i as u16),
                Print(line)
            )?;
        }
        // If the machine has requested input, show the prompt and the input buffer.
        if let Some(prompt) = &machine.input_request {
            execute!(
                stdout,
                cursor::MoveTo(0, output_panel_start + output_panel_height - 1),
                Print(format!("{} {}", prompt, input_buffer))
            )?;
        }

        // --- Bottom Info: Display the Program Counter ---
        execute!(
            stdout,
            cursor::MoveTo(0, rows.saturating_sub(1)),
            Print(format!("Program Counter: {}", machine.get_program_counter()))
        )?;

        stdout.flush()?;

        // If the program has halted, wait for the user to press 'q' to quit.
        if machine.get_program_counter() >= machine.get_program().len() {
            execute!(
                stdout,
                cursor::MoveTo(0, rows.saturating_sub(2)),
                Print("Program halted. Press 'q' to quit.")
            )?;
            stdout.flush()?;
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if code == KeyCode::Char('q') {
                    break;
                }
            }
            continue;
        }

        // Event handling:
        if event::poll(Duration::from_millis(500))? {
            let event = event::read()?;
            match event {
                Event::Key(key_event) => {
                    // If the machine is waiting for input...
                    if machine.input_request.is_some() {
                        match key_event.code {
                            KeyCode::Enter => {
                                // Try to parse the input.
                                match input_buffer.trim().parse::<i32>() {
                                    Ok(num) => {
                                        machine.push_value(num);
                                        machine.output.push(format!("Read: {}", num));
                                    }
                                    Err(_) => {
                                        machine.output.push("Invalid input. Please enter a number.".to_string());
                                    }
                                }
                                machine.input_request = None;
                                input_buffer.clear();
                            }
                            KeyCode::Char(c) => {
                                // Only process key press events.
                                if key_event.kind == KeyEventKind::Press {
                                    input_buffer.push(c);
                                }
                            }
                            KeyCode::Backspace => {
                                input_buffer.pop();
                            }
                            _ => {}
                        }
                    } else {
                        // Normal mode: handle stepping and quitting.
                        if key_event.kind == KeyEventKind::Press {
                            match key_event.code {
                                KeyCode::Char('n') => {
                                    machine.step();
                                }
                                KeyCode::Char('q') => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Clean up: restore the terminal.
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
