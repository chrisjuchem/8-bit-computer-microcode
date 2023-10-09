#![feature(const_option)]
#![feature(const_trait_impl)]

use microcode::instructions::Instructions;
use microcode::operations::Operations;
use microcode::programs::{Program, ADDITION, EMPTY};
use std::fmt::Display;
use std::fs::File;
use std::io::{Error, Write};

fn write_bytes<const N: usize, I: Display>(buf: [I; N], mut f: File) -> Result<(), Error> {
    f.write("{".as_bytes())?;
    for i in buf.iter() {
        f.write(i.to_string().as_bytes())?;
        f.write(",".as_bytes())?;
    }
    f.write("}".as_bytes())?;
    Ok(())
}

fn write_microcode() {
    let mut buf = [0u16; 2048];
    let noop = Instructions::microcode(Instructions::NOOP.opcode()).unwrap();

    for i in 0..=255 {
        let code = Instructions::microcode(i).unwrap_or(noop);
        code.steps().enumerate().for_each(|(n, instr)| {
            instr.validate().unwrap();
            buf[n * 256 + (i as usize)] = instr.machine_bits();
        })
    }

    let f = File::create("../generated/microcode.txt").unwrap();
    write_bytes(buf, f).unwrap();
}

fn write_program<const N: usize>(prog: Program<N>) {
    let mut bytes = [0; 256];
    prog.copy_into(&mut bytes);

    let f = File::create(format!("../generated/programs/{}.txt", prog.name)).unwrap();
    write_bytes(bytes, f).unwrap();
}

fn main() -> Result<(), Operations> {
    write_microcode();
    write_program(ADDITION);
    write_program(EMPTY);

    let debug_prog: [u8; 256] = (0..=255u8)
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let f = File::create("../generated/programs/debug.txt").unwrap();
    write_bytes(debug_prog, f).unwrap();

    Ok(())
}
