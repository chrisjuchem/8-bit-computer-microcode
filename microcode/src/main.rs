#![feature(const_option)]
#![feature(const_trait_impl)]

mod instructions;
mod operations;

use crate::instructions::Instructions;
use operations::Operations;

fn microcode() {
    for i in 0u8..=255 {
        if let Some(code) = Instructions::microcode(i) {
            println!("    case {}:", i);
            println!("      return {};", code);
        }
    }

    println!("    default:",);
    println!(
        "      return {};",
        Instructions::microcode(Instructions::NOOP.opcode()).unwrap()
    );
}

fn programs() {
    let mut prog = [0; 256];
    let mut prog_ref = &mut prog[..];

    prog_ref = Instructions::LDAI { val: 16 }.write_bytes(prog_ref);
    prog_ref = Instructions::LDBI { val: 1 }.write_bytes(prog_ref);
    prog_ref = Instructions::ADDA.write_bytes(prog_ref);
    prog_ref = Instructions::ADDA.write_bytes(prog_ref);
    prog_ref = Instructions::ADDA.write_bytes(prog_ref);
    prog_ref = Instructions::ADDA.write_bytes(prog_ref);
    prog_ref = Instructions::ADDA.write_bytes(prog_ref);
    Instructions::HALT.write_bytes(prog_ref);

    for i in 0..=255 {
        println!("    case {}:", i);
        println!("      return Instruction{{ {},0,0,0,0,0,0,0 }};", prog[i]);
    }
}

fn main() -> Result<(), Operations> {
    programs();

    Ok(())
}
