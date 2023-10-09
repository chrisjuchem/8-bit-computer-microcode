use microcode::instructions::{Instruction, Instructions};
use microcode::operations::Operations;
use microcode::programs::Program;
use std::fmt::{Display, Formatter};

pub struct State {
    a: u8,
    b: u8,
    pc: u8,
    step: u8,
    ir: u8,
    mar: u8,
    rom: [u8; 240],
    ram: [u8; 16],
}

impl State {
    pub fn new<const N: usize>(prog: Program<N>) -> State {
        let mut rom = [0; 240];
        prog.copy_into(&mut rom);

        State {
            a: 0,
            b: 0,
            pc: 0,
            step: 0,
            ir: 0,
            mar: 0,
            rom,
            ram: [0; 16],
        }
    }
}

impl State {
    pub fn step(&mut self) -> Result<(), ()> {
        let ops = Instructions::microcode(self.ir)
            .unwrap()
            .step(self.step as usize);

        if ops.contains(Operations::HL) {
            return Err(());
        }

        let bus = match ops.intersection(Operations::OUTPUTS).into_iter().next() {
            Some(Operations::CO) => self.pc,
            Some(Operations::AO) => self.a,
            Some(Operations::BO) => self.b,
            Some(Operations::RO) => self.mem(),
            Some(Operations::EO) => {
                if ops.contains(Operations::SU) {
                    self.a.wrapping_sub(self.b)
                } else {
                    self.a.wrapping_add(self.b)
                }
            }
            _ => 0,
        };

        if ops.contains(Operations::II) {
            self.ir = bus;
        }
        if ops.contains(Operations::MI) {
            self.mar = bus;
        }
        if ops.contains(Operations::RI) {
            self.ram[240 - self.mar as usize] = bus;
        }
        if ops.contains(Operations::AI) {
            self.a = bus;
        }
        if ops.contains(Operations::BI) {
            self.b = bus;
        }
        // if ops.contains(Operations::OI) {
        //     self.out = bus;
        // }

        if ops.contains(Operations::CE) {
            self.pc += 1;
        }

        self.step += 1;
        if self.step > 7 || ops.contains(Operations::SR) {
            self.step = 0;
        }
        Ok(())
    }

    fn mem(&self) -> u8 {
        match self.mar {
            a if a < 240 => self.rom[a as usize],
            a => self.ram[a as usize],
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mem = if self.mar < 240 { "rom" } else { "ram" };

        write!(f, "mar:{:#04X} {}:{:#04X}", self.mar, mem, self.mem())?;
        write!(f, "         ")?;
        write!(f, "a:{:#04X} b:{:#04X}", self.a, self.b)?;
        write!(f, "\n")?;
        write!(f, "ir :{:#04X} step:  {}", self.ir, self.step)?;
        write!(f, "        ")?;
        write!(f, "pc:{:#04X}", self.pc)?;
        write!(f, "\n")?;
        write!(
            f,
            "{:?}",
            Instructions::microcode(self.ir)
                .unwrap()
                .step(self.step as usize)
        )?;
        write!(f, "\n")?;
        Ok(())
    }
}
