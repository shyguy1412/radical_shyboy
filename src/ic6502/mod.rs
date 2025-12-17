mod opcodes;

use serde_derive::{Deserialize, Serialize};

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
    #[serde(rename = "px")]
    program_pointer: u16,
    #[serde(rename = "s")]
    status: u8,
}