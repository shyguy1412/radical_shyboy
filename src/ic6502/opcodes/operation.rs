use std::ops::Add;

use serde::de::value;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    bus::OpenBus,
    ic6502::{Flags, IC6502, STACK_PAGE, set_flag, unset_flag},
};

///Represents the Location of some Data
///
///Either a byte in a Register or an Address from where to read/write
pub enum OperationArgument {
    Value(u8),
    Pointer(u16),
}
use OperationArgument::*;

#[derive(Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "ADC")]
    AddToAccumulator,
    #[serde(rename = "SBC")]
    SubtractFromAccumulator,

    #[serde(rename = "INC")]
    Increment,
    #[serde(rename = "INX")]
    IncrementIndexX,
    #[serde(rename = "INY")]
    IncrementIndexY,
    #[serde(rename = "DEC")]
    Decrement,
    #[serde(rename = "DEX")]
    DecrementIndexX,
    #[serde(rename = "DEY")]
    DecrementIndexY,

    #[serde(rename = "AND")]
    BitwiseANDAccumulator,
    #[serde(rename = "EOR")]
    BitwiseXORAccumulator,
    #[serde(rename = "ORA")]
    BitwiseORAccumulator,

    #[serde(rename = "ASL")]
    LeftShift,
    #[serde(rename = "LSR")]
    RightShift,
    #[serde(rename = "ROL")]
    RotateBitLeft,
    #[serde(rename = "ROR")]
    RotateBitRight,

    #[serde(rename = "BCC")]
    BranchOnCarryClear,
    #[serde(rename = "BCS")]
    BranchOnCarrySet,
    #[serde(rename = "BEQ")]
    BranchOnResultZero,
    #[serde(rename = "BMI")]
    BranchOnResultMinus,
    #[serde(rename = "BNE")]
    BranchOnResultNotZero,
    #[serde(rename = "BPL")]
    BranchOnResultPlus,
    #[serde(rename = "BVC")]
    BranchOnOverflowClear,
    #[serde(rename = "BVS")]
    BranchOnOverflowSet,

    #[serde(rename = "BIT")]
    TestBitsWithAccumulator,

    #[serde(rename = "CLC")]
    ClearCarryFlag,
    #[serde(rename = "CLD")]
    ClearDecimalMode,
    #[serde(rename = "CLI")]
    ClearInterruptDisableBit,
    #[serde(rename = "CLV")]
    ClearOverflowFlag,

    #[serde(rename = "CMP")]
    ComapareWithAccumulator,
    #[serde(rename = "CPX")]
    CompareWithIndexX,
    #[serde(rename = "CPY")]
    CompareWithIndexY,

    #[serde(rename = "JMP")]
    Jump,
    #[serde(rename = "JSR")]
    JumpToSubRoutine,

    #[serde(rename = "LDA")]
    LoadToAccumulator,
    #[serde(rename = "LDX")]
    LoadToXRegister,
    #[serde(rename = "LDY")]
    LoadToYRegister,

    #[serde(rename = "PHA")]
    PushAccumulatorToStack,
    #[serde(rename = "PHP")]
    PushStatusToStack,
    #[serde(rename = "PLA")]
    PullAccumulatorFromStack,
    #[serde(rename = "PLP")]
    PullStatusFromStack,

    #[serde(rename = "RTI")]
    ReturnFromInterrupt,
    #[serde(rename = "RTS")]
    ReturnFromSubroutine,
    #[serde(rename = "SEC")]
    SetCarryFlag,
    #[serde(rename = "SED")]
    DetDecimalMode,
    #[serde(rename = "SEI")]
    SetInterruptStatus,

    #[serde(rename = "STA")]
    StoreAccumulator,
    #[serde(rename = "STX")]
    StoreXRegister,
    #[serde(rename = "STY")]
    StoreYRegister,

    #[serde(rename = "TAX")]
    TransferAccumulatorToX,
    #[serde(rename = "TAY")]
    TransferAccumulatorToY,
    #[serde(rename = "TSX")]
    TransferStackPointerToX,
    #[serde(rename = "TXA")]
    TransferXToAccumulator,
    #[serde(rename = "TXS")]
    TransferXToStackRegister,
    #[serde(rename = "TYA")]
    TransferYToAccumulator,

    #[serde(rename = "BRK")]
    ForceBreak,

    #[serde(rename = "NOP")]
    NoOp,
}

type OperationResult = Option<Thingimagic>;

pub enum Thingimagic {
    Jump(u16),
    Increment,
}

use Thingimagic::*;

impl Operation {
    pub fn run(
        &self,
        cpu: &mut IC6502,
        bus: &mut impl OpenBus,
        arg: OperationArgument,
    ) -> OperationResult {
        use Operation::*;
        match self {
            AddToAccumulator => operation_adc(cpu, bus, arg),
            SubtractFromAccumulator => operation_sbc(cpu, bus, arg),
            Increment => operation_inc(cpu, bus, arg),
            IncrementIndexX => operation_inx(cpu, bus, arg),
            IncrementIndexY => operation_iny(cpu, bus, arg),
            Decrement => operation_dec(cpu, bus, arg),
            DecrementIndexX => operation_dex(cpu, bus, arg),
            DecrementIndexY => operation_dey(cpu, bus, arg),
            BitwiseANDAccumulator => operation_and(cpu, bus, arg),
            BitwiseXORAccumulator => operation_eor(cpu, bus, arg),
            BitwiseORAccumulator => operation_ora(cpu, bus, arg),
            LeftShift => operation_asl(cpu, bus, arg),
            RightShift => operation_lsr(cpu, bus, arg),
            RotateBitLeft => operation_rol(cpu, bus, arg),
            RotateBitRight => operation_ror(cpu, bus, arg),
            BranchOnCarryClear => operation_bcc(cpu, bus, arg),
            BranchOnCarrySet => operation_bcs(cpu, bus, arg),
            BranchOnResultZero => operation_beq(cpu, bus, arg),
            BranchOnResultMinus => operation_bmi(cpu, bus, arg),
            BranchOnResultNotZero => operation_bne(cpu, bus, arg),
            BranchOnResultPlus => operation_bpl(cpu, bus, arg),
            BranchOnOverflowClear => operation_bvc(cpu, bus, arg),
            BranchOnOverflowSet => operation_bvs(cpu, bus, arg),
            TestBitsWithAccumulator => operation_bit(cpu, bus, arg),
            ClearCarryFlag => operation_clc(cpu, bus, arg),
            ClearDecimalMode => operation_cld(cpu, bus, arg),
            ClearInterruptDisableBit => operation_cli(cpu, bus, arg),
            ClearOverflowFlag => operation_clv(cpu, bus, arg),
            ComapareWithAccumulator => operation_cmp(cpu, bus, arg),
            CompareWithIndexX => operation_cpx(cpu, bus, arg),
            CompareWithIndexY => operation_cpy(cpu, bus, arg),
            Jump => operation_jmp(cpu, bus, arg),
            JumpToSubRoutine => operation_jsr(cpu, bus, arg),
            LoadToAccumulator => operation_lda(cpu, bus, arg),
            LoadToXRegister => operation_ldx(cpu, bus, arg),
            LoadToYRegister => operation_ldy(cpu, bus, arg),
            PushAccumulatorToStack => operation_pha(cpu, bus, arg),
            PushStatusToStack => operation_php(cpu, bus, arg),
            PullAccumulatorFromStack => operation_pla(cpu, bus, arg),
            PullStatusFromStack => operation_plp(cpu, bus, arg),
            ReturnFromInterrupt => operation_rti(cpu, bus, arg),
            ReturnFromSubroutine => operation_rts(cpu, bus, arg),
            SetCarryFlag => operation_sec(cpu, bus, arg),
            DetDecimalMode => operation_sed(cpu, bus, arg),
            SetInterruptStatus => operation_sei(cpu, bus, arg),
            StoreAccumulator => operation_sta(cpu, bus, arg),
            StoreXRegister => operation_stx(cpu, bus, arg),
            StoreYRegister => operation_sty(cpu, bus, arg),
            TransferAccumulatorToX => operation_tax(cpu, bus, arg),
            TransferAccumulatorToY => operation_tay(cpu, bus, arg),
            TransferStackPointerToX => operation_tsx(cpu, bus, arg),
            TransferXToAccumulator => operation_txa(cpu, bus, arg),
            TransferXToStackRegister => operation_txs(cpu, bus, arg),
            TransferYToAccumulator => operation_tya(cpu, bus, arg),
            ForceBreak => operation_brk(cpu, bus, arg),
            NoOp => operation_nop(cpu, bus, arg),
        }
    }
}

// Operations
#[inline(always)]
fn operation_adc(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_sbc(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    let value = match argument {
        OperationArgument::Value(v) => v,
        OperationArgument::Pointer(p) => bus.read(p)?,
    };

    let value = !value.wrapping_add(1);

    Some(Increment)
}

#[inline(always)]
fn operation_inc(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_inx(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_iny(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_dec(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_dex(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_dey(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_and(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_eor(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_ora(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    let value = match argument {
        Value(v) => v,
        Pointer(p) => bus.read(p)?,
    };

    cpu.accumulator |= value;

    set_flag!(cpu.status, Zero, cpu.accumulator == 0);

    set_flag!(
        cpu.status,
        Negative,
        Flags::Negative.is_set(cpu.accumulator)
    );

    Some(Increment)
}

#[inline(always)]
fn operation_asl(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    let value = match argument {
        Value(v) => v,
        Pointer(p) => bus.read(p)?,
    };

    set_flag!(cpu.status, Carry, Flags::Negative.is_set(value));

    let value = value << 1;

    set_flag!(cpu.status, Negative, Flags::Negative.is_set(value));
    set_flag!(cpu.status, Zero, value == 0);

    match argument {
        Value(_) => cpu.accumulator = value,
        Pointer(p) => bus.write(p, value)?,
    }

    Some(Increment)
}

#[inline(always)]
fn operation_lsr(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_rol(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_ror(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bcc(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bcs(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_beq(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bmi(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bne(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bpl(
    cpu: &mut IC6502,
    _: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    if Flags::Negative.is_set(cpu.status) {
        return Some(Increment);
    };

    let Value(value) = argument else {
        return None;
    };

    let value = value.cast_signed() as i16;

    let location = cpu.program_counter.wrapping_add(2);

    match value.is_negative() {
        true => Some(Jump(location.wrapping_sub(value.unsigned_abs()))),
        false => Some(Jump(location.wrapping_add(value.unsigned_abs()))),
    }
}

#[inline(always)]
fn operation_bvc(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bvs(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_bit(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_clc(cpu: &mut IC6502, _: &mut impl OpenBus, _: OperationArgument) -> OperationResult {
    unset_flag!(cpu.status, Carry);
    Some(Increment)
}

#[inline(always)]
fn operation_cld(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_cli(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_clv(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_cmp(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_cpx(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_cpy(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_jmp(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_jsr(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_lda(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_ldx(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_ldy(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_pha(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    let Value(value) = argument else {
        return None;
    };
    let stack_addr = STACK_PAGE.wrapping_add(cpu.stack_pointer as u16);
    bus.write(stack_addr, value)?;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
    Some(Increment)
}

#[inline(always)]
fn operation_php(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    _: OperationArgument,
) -> OperationResult {
    let addr = (cpu.stack_pointer as u16).wrapping_add(0x0100);

    bus.write(addr, cpu.status | Flags::Break);

    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);

    Some(Increment)
}

#[inline(always)]
fn operation_pla(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_plp(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_rti(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_rts(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_sec(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_sed(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_sei(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_sta(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_stx(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_sty(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_tax(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_tay(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_tsx(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_txa(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_txs(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_tya(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    argument: OperationArgument,
) -> OperationResult {
    Some(Increment)
}

#[inline(always)]
fn operation_brk(
    cpu: &mut IC6502,
    bus: &mut impl OpenBus,
    _: OperationArgument,
) -> OperationResult {
    let _ = bus.read(cpu.program_counter.wrapping_add(1));
    let [low_byte, high_byte] = (cpu.program_counter.wrapping_add(2)).to_le_bytes();
    operation_pha(cpu, bus, Value(high_byte))?;
    operation_pha(cpu, bus, Value(low_byte))?;
    operation_pha(cpu, bus, Value(cpu.status | Flags::Break))?;
    cpu.status |= Flags::InterruptDisable;
    Some(Jump(u16::from_le_bytes([
        bus.read(0xFFFE)?,
        bus.read(0xFFFF)?,
    ])))
}

#[inline(always)]
fn operation_nop(_: &mut IC6502, _: &mut impl OpenBus, _: OperationArgument) -> OperationResult {
    Some(Increment)
}
