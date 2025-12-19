mod opcodes;

use opcodes::Instruction;
use serde_derive::{Deserialize, Serialize};

use crate::bus::{OpenBus, OpenBusDevice};

/// Represents the State of the 6502 Mikroprocessor
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct IC6502 {
    #[serde(rename = "a")]
    accumulator: u8,
    #[serde(rename = "x")]
    register_x: u8,
    #[serde(rename = "y")]
    register_y: u8,
    #[serde(rename = "p")]
    stack_pointer: u8,
    #[serde(rename = "pc")]
    program_counter: u16,
    #[serde(rename = "s")]
    status: u8,
}

#[allow(unused, dropping_copy_types)]
impl<B: OpenBus> OpenBusDevice<B> for IC6502 {
    fn cycle(&mut self, bus: &mut B) -> u8 {
        //no matter what happens, progam counter always needs to increment at least once
        self.program_counter += 1;

        let Some(instruction) = bus.read(self.program_counter - 1) else {
            // ! reading from open bus
            // ! currently defined as noop
            return 0;
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
            return 0;
        };

        let Some((offset, argument)) = addressing_mode.read(self, bus) else {
            // ! read operation from open bus
            return 0;
        };
        self.program_counter += offset as u16;

        // operation.run(self, bus);

        drop(0);
        1
    }
}
