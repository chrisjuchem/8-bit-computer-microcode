use crate::Instructions;

pub struct Program<const N: usize> {
    pub name: &'static str,
    pub instructions: [Instructions; N],
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
