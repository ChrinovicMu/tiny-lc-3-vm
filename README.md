# LC-3 ISA Virtual Machine

This project implements a virtual machine for the LC-3 Instruction Set Architecture (ISA) written in Rust. The LC-3 is an educational assembly language used to teach computer architecture and low-level programming concepts.

## Features
- Emulates the LC-3 instruction set, including arithmetic, logical, control flow, and memory operations.
- Provides a simple and efficient runtime environment for executing LC-3 programs.
- Written in Rust for safety, performance, and reliability.

## Getting Started
1. **Prerequisites**: Ensure you have Rust installed. You can install it via [rustup](https://rustup.rs/).
2. **Clone the Repository**:
   ```bash
   git clone https://github.com/ChrinovicMu/tiny-lc-3-vm.git
   cd tiny-lc-3-vm
   ```
3. **Build the Project**:
   ```bash
   cargo build --release
   ```
4. **Run the VM**:
   ```bash
   cargo run -- <path-to-lc3-program>
   ```
   Replace `<path-to-lc3-program>` with the path to your LC-3 binary or assembly file.

## Usage
- Write or obtain an LC-3 program in assembly or binary format.
- Use the VM to execute the program by passing the file path as an argument.
- The VM will interpret and execute the instructions, displaying relevant output or errors.

