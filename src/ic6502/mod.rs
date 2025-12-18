mod opcodes;

use serde_derive::{Deserialize, Serialize};

use crate::{
    bus::{Bus, OpenBus, OpenBusDevice},
    ic6502::opcodes::{Instruction, Operation},
};

/// Represents the State of the 6502 Mikroprocessor
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct IC6502 {
    #[serde(rename = "a")]
    register_a: u8,
    #[serde(rename = "x")]
    register_x: u8,
    #[serde(rename = "y")]
    register_y: u8,
    #[serde(rename = "p")]
    stack_pointer: u8,
    #[serde(rename = "pc")]
    program_pointer: u16,
    #[serde(rename = "s")]
    status: u8,
}

impl<B: OpenBus> OpenBusDevice<B> for IC6502 {
    fn cycle(&mut self, bus: &mut B) {
        //no matter what happens, progam counter always needs to increment
        self.program_pointer += 1;

        let Some(instruction) = bus.read(self.program_pointer - 1) else {
            // ! reading from open bus
            // ! currently defined as noop
            return;
        };

        let Instruction::Valid {
            operation,
            addressing_mode,
            bytes,
            cycles,
        } = instruction.into()
        else {
            // ! invalid instruction
            // ! currently defined as noop
            return;
        };

        drop(0)
    }
}
