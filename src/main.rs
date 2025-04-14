extern crate termios;

pub mod hardware;
use hardware::vm::VM;

use termios::*;

use byteorder::{BigEndian, ReadBytesExt};

use std::{fs::File, io::BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(long)]
    print_asm: bool, // Future feature
}

fn main() {


    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    // make a mutable copy of termios
    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    // Actual VM logic code
    let mut vm = VM::new();

    let cli = Cli::from_args();

    let f = File::open(cli.path).expect("couldn't open file");
    let mut f = BufReader::new(f);

    
    let base_address = f.read_u16::<BigEndian>().expect("error");


    let mut address = base_address as usize;
    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instruction) => {
                vm.write_memory(address, instruction);
                address += 1;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("OK")
                } else {
                    println!("failed: {}", e);
                }
                break;
            }
        }
    }

    hardware::execute_program(&mut vm);

    // reset the stdin to orginal termio 
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
