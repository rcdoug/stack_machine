# Implementation of a Stack Machine using Rust

# Architecture Overview

TODO

# Structure

TODO

# Command List

Stack Operations: `push`, `pop`, `dup`, `swap`, `rot`, `clear`

Arithmetic: `add`, `sub`, `mul`, `div`, `rem`, `neg`, `inc`, `dec`

Comparison: `eq`, `ne`, `le`, `ge`, `lt`, `gt`

Control Flow: `jump`, `brt`, `brz`, `halt`, `label`

Memory: `load`, `save`, `store`, `alloc`, `free`

I/O: `print`, `read`, `write`, `scan`

Bitwise/Logical: `and`, `or`, `xor`, `not`, `shl`, `shr`, `bool`

Debugging: `dump`, `trace`

# Command Usage and Descriptions

## Stack Operations
- **`push <value>`**  
  Push a value onto the stack.  
  *Example*: `push 10` → Stack: `[10]`

- **`pop`**  
  Remove the top value from the stack.  
  *Example*: Stack: `[10, 20]` → `pop` → Stack: `[10]`

- **`dup`**  
  Duplicate the top value of the stack.  
  *Example*: Stack: `[10]` → `dup` → Stack: `[10, 10]`

- **`swap`**  
  Swap the top two values on the stack.  
  *Example*: Stack: `[10, 20]` → `swap` → Stack: `[20, 10]`

- **`rot`**  
  Rotate the top three values on the stack.  
  *Example*: Stack: `[10, 20, 30]` → `rot` → Stack: `[30, 10, 20]`

- **`clear`**  
  Clear all values from the stack.  
  *Example*: Stack: `[10, 20]` → `clear` → Stack: `[]`

## Arithmetic
- **`add`**  
  Add the top two values on the stack.  
  *Example*: Stack: `[10, 20]` → `add` → Stack: `[30]`

- **`sub`**  
  Subtract the top value from the second value on the stack.  
  *Example*: Stack: `[10, 20]` → `sub` → Stack: `[-10]`

- **`mul`**  
  Multiply the top two values on the stack.  
  *Example*: Stack: `[10, 20]` → `mul` → Stack: `[200]`

- **`div`**  
  Divide the second value by the top value on the stack.  
  *Example*: Stack: `[20, 10]` → `div` → Stack: `[2]`

- **`rem`**  
  Compute the remainder of the second value divided by the top value.  
  *Example*: Stack: `[20, 6]` → `rem` → Stack: `[2]`

- **`neg`**  
  Negate the top value on the stack.  
  *Example*: Stack: `[10]` → `neg` → Stack: `[-10]`

- **`inc`**  
  Increment the top value of the stack by 1.  
  *Example*: Stack: `[10]` → `inc` → Stack: `[11]`

- **`dec`**  
  Decrement the top value of the stack by 1.  
  *Example*: Stack: `[10]` → `dec` → Stack: `[9]`

## Comparison
- **`eq`**  
  Compare the top two values; push `1` if equal, otherwise `0`.  
  *Example*: Stack: `[10, 10]` → `eq` → Stack: `[1]`

- **`ne`**  
  Compare the top two values; push `1` if not equal, otherwise `0`.  
  *Example*: Stack: `[10, 20]` → `ne` → Stack: `[1]`

- **`le`**  
  Push `1` if the second value is less than or equal to the top value, otherwise `0`.  
  *Example*: Stack: `[10, 20]` → `le` → Stack: `[1]`

- **`ge`**  
  Push `1` if the second value is greater than or equal to the top value, otherwise `0`.  
  *Example*: Stack: `[20, 10]` → `ge` → Stack: `[1]`

- **`lt`**  
  Push `1` if the second value is less than the top value, otherwise `0`.  
  *Example*: Stack: `[10, 20]` → `lt` → Stack: `[1]`

- **`gt`**  
  Push `1` if the second value is greater than the top value, otherwise `0`.  
  *Example*: Stack: `[20, 10]` → `gt` → Stack: `[1]`

## Control Flow
- **`jump <label>`**  
  Jump to the specified label.  
  *Example*: `jump loopStart`

- **`brt <label>`**  
  Branch to the label if the top value is true (nonzero).  
  *Example*: Stack: `[1]` → `brt myLabel`

- **`brz <label>`**  
  Branch to the label if the top value is false (zero).  
  *Example*: Stack: `[0]` → `brz myLabel`

- **`halt`**  
  Stop execution of the program.

- **`label <name>`**  
  Define a label to mark a position in the program.  
  *Example*: `label loopStart`

## Memory
- **`load <address>`**  
  Push the value at the specified memory address onto the stack.  
  *Example*: `load 0x100`

- **`store <address>`**  
  Store the top value of the stack at the specified memory address.  
  *Example*: Stack: `[42]` → `store 0x100`

- **`alloc <size>`**  
  Allocate a block of memory of the specified size.  
  *Example*: `alloc 10`

- **`free <address>`**  
  Free a previously allocated memory block.  
  *Example*: `free 0x100`

---

## I/O
- **`print`**  
  Print the top value of the stack.  
  *Example*: Stack: `[42]` → `print` → Output: `42`

- **`read`**  
  Read a single input value and push it onto the stack.

- **`write <string>`**  
  Output a string or formatted text.  
  *Example*: `write "Hello, world!"`

- **`scan`**  
  Read a full line of input and push it onto the stack as a string.

## Bitwise/Logical
- **`and`**  
  Perform a bitwise AND on the top two values.  
  *Example*: Stack: `[1, 3]` → `and` → Stack: `[1]`

- **`or`**  
  Perform a bitwise OR on the top two values.  
  *Example*: Stack: `[1, 2]` → `or` → Stack: `[3]`

- **`xor`**  
  Perform a bitwise XOR on the top two values.  
  *Example*: Stack: `[1, 3]` → `xor` → Stack: `[2]`

- **`not`**  
  Perform a bitwise NOT on the top value.  
  *Example*: Stack: `[0xF0]` → `not` → Stack: `[0x0F]`

- **`shl`**  
  Shift the second value left by the top value bits.  
  *Example*: Stack: `[1, 3]` → `shl` → Stack: `[6]`

- **`shr`**  
  Shift the second value right by the top value bits.  
  *Example*: Stack: `[4, 1]` → `shr` → Stack: `[2]`

- **`bool`**  
  Convert the top value to a boolean (0 → `0`, nonzero → `1`).  
  *Example*: Stack: `[42]` → `bool` → Stack: `[1]`

## Debugging
- **`dump`**  
  Print the entire stack or a specified memory region.  
  *Example*: `dump`

- **`trace <on/off>`**  
  Enable or disable execution tracing.  
  *Example*: `trace on`
