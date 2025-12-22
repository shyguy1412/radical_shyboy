use serde_derive::{Deserialize, Serialize};

use crate::bus::{OpenBus, OpenBusDevice};

mod flags;
pub use flags::*;

mod opcodes;
pub use opcodes::Instruction;
use opcodes::Thingimagic;

/// Represents the State of the 6502 Mikroprocessor
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct IC6502 {
    #[serde(rename = "a")]
    accumulator: u8,
    #[serde(rename = "x")]
    register_x: u8,
    #[serde(rename = "y")]
    register_y: u8,
    #[serde(rename = "s")]
    stack_pointer: u8,
    #[serde(rename = "pc")]
    program_counter: u16,
    #[serde(rename = "p")]
    status: u8,
}

pub const STACK_PAGE: u16 = 0x0100;

#[allow(unused, dropping_copy_types)]
impl<B: OpenBus> OpenBusDevice<B> for IC6502 {
    fn cycle(&mut self, bus: &mut B) -> Option<u8> {
        let instruction = bus.read(self.program_counter)?;

        let Instruction::Valid {
            operation,
            addressing_mode,
            bytes,
            cycles,
        } = instruction.into()
        else {
            // ! invalid instruction
            // ! currently defined as noop
            self.program_counter = self.program_counter.wrapping_add(1);
            return None;
        };

        let (offset, argument) = addressing_mode.read(self, bus)?;

        match operation.run(self, bus, argument)? {
            Thingimagic::Jump(ptr) => self.program_counter = ptr,
            Thingimagic::Increment => {
                self.program_counter = self.program_counter.wrapping_add(offset as u16)
            }
        };

        drop(0);
        Some(1) //eventually this should return the count of cycles this instruction would have taken
    }
}
