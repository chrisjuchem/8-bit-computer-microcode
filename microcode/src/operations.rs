use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Operations: u32 {
        /// Program Counter Out
        const CO = 1 << 30;
        /// RAM Out
        const RO = 1 << 29;
        /// unused
        const _1 = 1 << 28;
        /// unused
        const _2 = 1 << 27;
        /// Sum Out
        const EO = 1 << 26;
        /// A Register Out
        const AO = 1 << 25;
        /// B Register Out
        const BO = 1 << 24;
        /// Jump (unconditional)
        const J_ = 1 << 18;
        /// Jump if Zero Flag Set
        const JZ = 1 << 17;
        /// Jump if Carry Flag Set
        const JC = 1 << 16;
        /// Instruction Register In
        const II = 1 << 10;
        /// Memory Address Register In
        const MI = 1 << 9;
        /// RAM In
        const RI = 1 << 8;
        /// A Register In
        const AI = 1 << 7;
        /// B Register In
        const BI = 1 << 6;
        /// Output Register In
        const OI = 1 << 5;
        /// unused
        const _3 = 1 << 4;
        /// Step Reset
        const SR = 1 << 3;
        /// Program Counter Enable
        const CE = 1 << 2;
        /// Subtract
        const SU = 1 << 1;
        /// Halt
        const HL = 1 << 0;

        const INSTR1 = Operations::CO.bits() | Operations::MI.bits();
        const INSTR2 = Operations::RO.bits() | Operations::II.bits() | Operations::CE.bits();
        const __ = 0;

        const OUTPUTS = 0b_0111_1111 << 24;
        const JUMPS = 0b_111 << 16;
    }
}

// pub const CO: Operations = Operations::CO;
// pub const RO: Operations = Operations::RO;
// pub const EO: Operations = Operations::EO;
// pub const AO: Operations = Operations::AO;
// pub const BO: Operations = Operations::BO;
// pub const J_: Operations = Operations::J_;
// pub const JZ: Operations = Operations::JZ;
// pub const JC: Operations = Operations::JC;
// pub const II: Operations = Operations::II;
// pub const MI: Operations = Operations::MI;
// pub const RI: Operations = Operations::RI;
// pub const AI: Operations = Operations::AI;
// pub const BI: Operations = Operations::BI;
// pub const OI: Operations = Operations::OI;
// pub const SR: Operations = Operations::SR;
// pub const CE: Operations = Operations::CE;
// pub const SU: Operations = Operations::SU;
// pub const HL: Operations = Operations::HL;

mod _asserts {
    use super::Operations;

    const _: () = {
        assert!(
            (Operations::CO.bits()
                | Operations::RO.bits()
                | Operations::_1.bits()
                | Operations::_2.bits()
                | Operations::EO.bits()
                | Operations::AO.bits()
                | Operations::BO.bits())
                == Operations::OUTPUTS.bits()
        );
        assert!(
            (Operations::J_.bits() | Operations::JZ.bits() | Operations::JC.bits())
                == Operations::JUMPS.bits()
        );
    };
}

impl Operations {
    pub fn machine_bits(&self) -> u16 {
        self.iter().fold(0, |collected, op| {
            collected
                | match op {
                    Operations::CO => 0b_00100000_00000000,
                    Operations::RO => 0b_01000000_00000000,
                    Operations::_1 => 0b_01100000_00000000,
                    Operations::_2 => 0b_10000000_00000000,
                    Operations::EO => 0b_10100000_00000000,
                    Operations::AO => 0b_11000000_00000000,
                    Operations::BO => 0b_11100000_00000000,
                    Operations::J_ => 0b_00001000_00000000,
                    Operations::JZ => 0b_00010000_00000000,
                    Operations::JC => 0b_00011000_00000000,
                    Operations::II => 0b_00000100_00000000,
                    Operations::MI => 0b_00000010_00000000,
                    Operations::RI => 0b_00000001_00000000,
                    Operations::AI => 0b_00000000_10000000,
                    Operations::BI => 0b_00000000_01000000,
                    Operations::OI => 0b_00000000_00100000,
                    Operations::_3 => 0b_00000000_00010000,
                    Operations::SR => 0b_00000000_00001000,
                    Operations::CE => 0b_00000000_00000100,
                    Operations::SU => 0b_00000000_00000010,
                    Operations::HL => 0b_00000000_00000001,
                    _ => 0,
                }
        })
    }

    pub fn validate(&self) -> Result<Self, Self> {
        let conflicting = [Operations::OUTPUTS, Operations::JUMPS];

        for conflict in conflicting {
            let ops = self.intersection(conflict);
            if ops.iter().count() > 1 {
                return Err(ops);
            }
        }

        Ok(*self)
    }
}
