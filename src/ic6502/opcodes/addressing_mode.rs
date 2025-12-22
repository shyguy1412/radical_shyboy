use serde_derive::{Deserialize, Serialize};

use crate::{
    bus::OpenBus,
    ic6502::{
        IC6502,
        opcodes::operation::OperationArgument::{self, *},
    },
};

#[derive(Serialize, Deserialize)]
pub enum AdressingMode {
    #[serde(rename = "IMP")]
    Implied,
    #[serde(rename = "IMM")]
    Immediate,
    #[serde(rename = "ACC")]
    Accumulator,
    #[serde(rename = "REL")]
    Relative,

    #[serde(rename = "ZP0")]
    ZeroPage,
    #[serde(rename = "ZPX")]
    IndexedZeroPageX,
    #[serde(rename = "ZPY")]
    IndexedZeroPageY,

    #[serde(rename = "ABS")]
    Absolute,
    #[serde(rename = "ABX")]
    IndexedAbsoluteX,
    #[serde(rename = "ABY")]
    IndexedAbsoluteY,

    #[serde(rename = "INX")]
    IndexedIndirect,
    #[serde(rename = "INY")]
    IndirectIndexed,
    #[serde(rename = "IND")]
    AbsoluteIndirect,
}

impl AdressingMode {
    /// Returns a tuple of the program counter offset caused by the read process
    /// and the operation argument that was read
    pub fn read(&self, cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
        use AdressingMode::*;
        match self {
            Implied => address_mode_imp(cpu, bus),
            Immediate => address_mode_imm(cpu, bus),
            Accumulator => address_mode_acc(cpu, bus),
            Relative => address_mode_rel(cpu, bus),

            ZeroPage => address_mode_zp0(cpu, bus),
            IndexedZeroPageX => address_mode_zpx(cpu, bus),
            IndexedZeroPageY => address_mode_zpy(cpu, bus),

            Absolute => address_mode_abs(cpu, bus),
            IndexedAbsoluteX => address_mode_abx(cpu, bus),
            IndexedAbsoluteY => address_mode_aby(cpu, bus),

            IndexedIndirect => address_mode_inx(cpu, bus),
            IndirectIndexed => address_mode_iny(cpu, bus),
            AbsoluteIndirect => address_mode_ind(cpu, bus),
        }
    }
}

/// Implied Adress mode will either not need any data at all or read from Accumulator
#[inline(always)]
fn address_mode_imp(cpu: &IC6502, _: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    Some((1, Value(cpu.accumulator)))
}

#[inline(always)]
fn address_mode_imm(cpu: &IC6502, _: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    Some((2, Pointer(cpu.program_counter.wrapping_add(1))))
}

#[inline(always)]
fn address_mode_acc(cpu: &IC6502, _: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    Some((1, Value(cpu.accumulator)))
}

#[inline(always)]
fn address_mode_rel(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    Some((2, Value(bus.read(cpu.program_counter.wrapping_add(1))?)))
}

#[inline(always)]
fn address_mode_zp0(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = bus.read(cpu.program_counter.wrapping_add(1))? as u16 & 0x00FF;
    Some((2, Pointer(addr)))
}

#[inline(always)]
fn address_mode_zpx(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = bus.read(cpu.program_counter.wrapping_add(1))?;
    let addr = addr.wrapping_add(cpu.register_x);
    Some((2, Pointer(addr as u16)))
}

#[inline(always)]
fn address_mode_zpy(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = bus.read(cpu.program_counter.wrapping_add(1))?;
    let addr = addr.wrapping_add(cpu.register_y);
    Some((2, Pointer(addr as u16)))
}

#[inline(always)]
fn address_mode_abs(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = u16::from_le_bytes([
        bus.read(cpu.program_counter.wrapping_add(1))?,
        bus.read(cpu.program_counter.wrapping_add(2))?,
    ]);
    Some((3, Pointer(addr)))
}

#[inline(always)]
fn address_mode_abx(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = u16::from_le_bytes([
        bus.read(cpu.program_counter.wrapping_add(1))?,
        bus.read(cpu.program_counter.wrapping_add(2))?,
    ]);
    let addr = addr.wrapping_add(cpu.register_x as u16);
    Some((3, Pointer(addr)))
}

#[inline(always)]
fn address_mode_aby(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = u16::from_le_bytes([
        bus.read(cpu.program_counter.wrapping_add(1))?,
        bus.read(cpu.program_counter.wrapping_add(2))?,
    ]);
    let addr = addr.wrapping_add(cpu.register_y as u16);
    Some((3, Pointer(addr)))
}

#[inline(always)]
fn address_mode_ind(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = u16::from_le_bytes([
        bus.read(cpu.program_counter.wrapping_add(1))?,
        bus.read(cpu.program_counter.wrapping_add(2))?,
    ]);

    //Emulate a hardware bug where at the page boundary instead of reading the next page
    //the address wraps around to the start of the page
    let addr = match addr {
        0x00FF => addr & 0x00FF,
        _ => addr,
    };

    let addr = u16::from_le_bytes([bus.read(addr)?, bus.read(addr.wrapping_add(1))?]);

    Some((3, Pointer(addr)))
}

#[inline(always)]
fn address_mode_inx(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = bus.read(cpu.program_counter.wrapping_add(1))?;
    let addr = addr.wrapping_add(cpu.register_x);

    let low = bus.read(addr as u16)?;
    let high = bus.read(addr.wrapping_add(1) as u16)?;

    let addr = u16::from_le_bytes([low, high]);
    Some((2, Pointer(addr)))
}

#[inline(always)]
fn address_mode_iny(cpu: &IC6502, bus: &impl OpenBus) -> Option<(u8, OperationArgument)> {
    let addr = bus.read(cpu.program_counter.wrapping_add(1))? as u8;
    let low = bus.read(addr as u16)?;
    let high = bus.read(addr.wrapping_add(1) as u16)?;
    let addr = u16::from_le_bytes([low, high]).wrapping_add(cpu.register_y as u16);
    Some((2, Pointer(addr)))
}
