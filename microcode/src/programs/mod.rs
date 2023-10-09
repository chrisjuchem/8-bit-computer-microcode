use crate::instructions::Instructions;

pub struct Program<const N: usize> {
    pub name: &'static str,
    pub instructions: [Instructions; N],
}

impl<const N: usize> Program<N> {
    pub fn copy_into(&self, mut buf: &mut [u8]) {
        for instr in self.instructions.iter() {
            buf = instr.write_bytes(buf)
        }
        while buf.len() > 0 {
            buf = Instructions::NOOP.write_bytes(buf);
        }
    }
}

pub const EMPTY: Program<0> = Program {
    name: "empty",
    instructions: [],
};

pub const ADDITION: Program<8> = Program {
    name: "addition",
    instructions: [
        Instructions::LDAI { val: 16 },
        Instructions::LDBI { val: 1 },
        Instructions::ADDA,
        Instructions::ADDA,
        Instructions::ADDA,
        Instructions::ADDA,
        Instructions::ADDA,
        Instructions::HALT,
    ],
};
