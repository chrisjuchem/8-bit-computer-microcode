use crate::Operations;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Instruction([Operations; 8]);
impl Instruction {
    pub fn steps(self) -> impl Iterator<Item = Operations> {
        self.0.into_iter()
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Instruction{")?;
        for i in 0..7 {
            f.write_str(&self.0[i].machine_bits().to_string())?;
            f.write_str(",")?;
        }
        f.write_str(&self.0[7].machine_bits().to_string())?;
        f.write_str("}")
    }
}

macro_rules! i {
    {[]} => {
        i!{@ [INSTR1, INSTR2, __, __, __, __, __, __]}
    };
    {[$( $o1:ident )|* $(,)?]} => {
        i!{@ [INSTR1, INSTR2, $($o1)|*, __, __, __, __, __]}
    };
    {[$( $o1:ident )|*, $( $o2:ident )|* $(,)?]} => {
        i!{@ [INSTR1, INSTR2, $($o1)|*, $($o2)|*, __, __, __, __]}
    };
    {[$($o1:ident )|*, $($o2:ident )|*, $($o3:ident )|* $(,)?]} => {
        i!{@ [INSTR1, INSTR2, $($o1 )|*, $($o2 )|*, $($o3 )|*, __, __, __]}
    };
    {[$($o1:ident )|*, $($o2:ident )|*, $($o3:ident )|*, $($o4:ident )|* $(,)?]} => {
        i!{@ [INSTR1, INSTR2, $($o1 )|*, $($o2 )|*, $($o3 )|*, $($o4 )|*, __, __]}
    };
    {[$($o1:ident )|*, $($o2:ident )|*, $($o3:ident )|*, $($o4:ident )|*, $($o5:ident )|* $(,)?]} => {
        i!{@ [INSTR1, INSTR2, $($o1 )|*, $($o2 )|*, $($o3 )|*, $($o4 )|*, $($o5 )|*, __]}
    };
    {[$($o1:ident )|*, $($o2:ident )|*, $($o3:ident )|*, $($o4:ident )|*, $($o5:ident )|*, $($o6:ident )|* $(,)?]} => {
        i!{@ [INSTR1, INSTR2, $($o1 )|*, $($o2 )|*, $($o3 )|*, $($o4 )|*, $($o5 )|*, $($o6 )|*]}
    };

    {@ [$( $o1:ident $( | $op:ident )* ),*]} => {
        Instruction([$( Operations::$o1 $(.union(Operations::$op))* ),*])
    };
}

// i! {NOOP, []}
// i! {LDAI, [CO|MI, RO|AI|CE]}

macro_rules! instructions {
    { $( $INS:ident $({ $($arg:tt:$ty:ty),* })? = $n:expr, $microcode:tt; )* } => {
        pub enum Instructions {
           $( $INS $({$($arg: $ty),*})?,)*
        }

        impl Instructions {
            pub fn opcode(&self) -> u8 {
                match self {
                    $(Instructions::$INS $({ $($arg),*: _ })? => $n),*
                }
            }

            pub fn microcode(n: u8) -> Option<Instruction> {
                match n {
                    $( $n => Some(i!{$microcode}), )*
                    _ => None
                }
            }

            pub fn write_bytes<'a>(&self, mut buf: &'a mut [u8]) -> &'a mut [u8] {
                buf[0] = self.opcode();
                buf = &mut buf[1..];
                match self {
                    $(Instructions::$INS $({ $($arg),* })? => {
                        $($(
                            buf[0] = *$arg;
                            buf = &mut buf[1..];
                        )*)?
                    },)*
                }
                buf
            }
        }
    };
}

instructions! {
    NOOP = 0, [];
    LDAI{val: u8} = 1, [CO|MI, RO|AI|CE];
    LDBI{val: u8} = 2, [CO|MI, RO|BI|CE];
    ADDA = 3, [EO|AI];
    HALT = 255, [HL];

    DEBUG = 200, [CO|II];
}
