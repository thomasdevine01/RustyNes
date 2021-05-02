

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Opcode{
    //Binary Operations
    ADC,
    SBC,
    AND,
    EOR,
    ORA,
    //Shifts and rotations
    ASL,
    LSR,
    ROL,
    ROR,
    //Increment/Decrement
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    // Load/Store
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    //Set/Clear flags
    SEC,
    SED,
    SEI,
    CLC,
    CLD,
    CLI,
    CLV,
    //Compare
    CMP,
    CPX,
    CPY,
    //Jump Return
    JMP,
    JSR,
    RTI,
    RTS,
    //Branch
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    //Push and Pop
    PHA,
    PHP,
    PLA,
    PLP,
    //Transfer
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    //Misc
    BRK,
    BIT,
    NOP,
    //Possibly will add unofficial opcodes but unlikely
    ALR,
    ANC,
    ARR,
    AXS,
    LAX,
    SAX,
    DCP,
    ISC,
    RLA,
    RRA,
    SLO,
    SRE,
    SKB,
    IGN,
}
#[derive(Clone, Copy)]
pub struct Operand(pub u16,  pub u8);
#[derive(Clone, Copy, Debug)]
pub struct Instruction(pub Opcode, pub AddressingMode);

impl Instruction {
    pub fn from(code : u8) -> Instruction {
        log(&code.to_string());
        match code {

            //BINARY OPERATIONS
            //ADC
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            0x65 => Instruction(Opcode::ADC, AddressingMode::ZeroPage),
            0x75 => Instruction(Opcode::ADC, AddressingMode::ZeroPageX),
            0x6d => Instruction(Opcode::ADC, AddressingMode::Absolute),
            0x7d => Instruction(Opcode::ADC, AddressingMode::AbsoluteX),
            0x79 => Instruction(Opcode::ADC, AddressingMode::AbsoluteY),
            0x61 => Instruction(Opcode::ADC, AddressingMode::IndirectX),
            0x71 => Instruction(Opcode::ADC, AddressingMode::IndirectY),

            //SBC
            0xe9 => Instruction(Opcode::SBC, AddressingMode::Immediate),
            0xe5 => Instruction(Opcode::SBC, AddressingMode::ZeroPage),
            0xf5 => Instruction(Opcode::SBC, AddressingMode::ZeroPageX),
            0xed => Instruction(Opcode::SBC, AddressingMode::Absolute),
            0xfd => Instruction(Opcode::SBC, AddressingMode::AbsoluteX),
            0xf9 => Instruction(Opcode::SBC, AddressingMode::AbsoluteY),
            0xe1 => Instruction(Opcode::SBC, AddressingMode::IndirectX),
            0xf1 => Instruction(Opcode::SBC, AddressingMode::IndirectY),

            //AND
            0x29 => Instruction(Opcode::AND, AddressingMode::Immediate),
            0x25 => Instruction(Opcode::AND, AddressingMode::ZeroPage),
            0x35 => Instruction(Opcode::AND, AddressingMode::ZeroPageX),
            0x2d => Instruction(Opcode::AND, AddressingMode::Absolute),
            0x3d => Instruction(Opcode::AND, AddressingMode::AbsoluteX),
            0x39 => Instruction(Opcode::AND, AddressingMode::AbsoluteY),
            0x21 => Instruction(Opcode::AND, AddressingMode::IndirectX),
            0x31 => Instruction(Opcode::AND, AddressingMode::IndirectY),

            //EOR
            0x49 => Instruction(Opcode::EOR, AddressingMode::Immediate),
            0x45 => Instruction(Opcode::EOR, AddressingMode::ZeroPage),
            0x55 => Instruction(Opcode::EOR, AddressingMode::ZeroPageX),
            0x4d => Instruction(Opcode::EOR, AddressingMode::Absolute),
            0x5d => Instruction(Opcode::EOR, AddressingMode::AbsoluteX),
            0x59 => Instruction(Opcode::EOR, AddressingMode::AbsoluteY),
            0x41 => Instruction(Opcode::EOR, AddressingMode::IndirectX),
            0x51 => Instruction(Opcode::EOR, AddressingMode::IndirectY),

            //ORA
            0x09 => Instruction(Opcode::ORA, AddressingMode::Immediate),
            0x05 => Instruction(Opcode::ORA, AddressingMode::ZeroPage),
            0x15 => Instruction(Opcode::ORA, AddressingMode::ZeroPageX),
            0x0d => Instruction(Opcode::ORA, AddressingMode::Absolute),
            0x1d => Instruction(Opcode::ORA, AddressingMode::AbsoluteX),
            0x19 => Instruction(Opcode::ORA, AddressingMode::AbsoluteY),
            0x01 => Instruction(Opcode::ORA, AddressingMode::IndirectX),
            0x11 => Instruction(Opcode::ORA, AddressingMode::IndirectY),


            //Shifts Rotates

            //ASL
            0x0a => Instruction(Opcode::ASL, AddressingMode::Accumulator),
            0x06 => Instruction(Opcode::ASL, AddressingMode::ZeroPage),
            0x16 => Instruction(Opcode::ASL, AddressingMode::ZeroPageX),
            0x0e => Instruction(Opcode::ASL, AddressingMode::Absolute),
            0x1e => Instruction(Opcode::ASL, AddressingMode::AbsoluteX),

            //LSR
            0x4a => Instruction(Opcode::LSR, AddressingMode::Accumulator),
            0x46 => Instruction(Opcode::LSR, AddressingMode::ZeroPage),
            0x56 => Instruction(Opcode::LSR, AddressingMode::ZeroPageX),
            0x4e => Instruction(Opcode::LSR, AddressingMode::Absolute),
            0x5e => Instruction(Opcode::LSR, AddressingMode::AbsoluteX),

            //ROL
            0x2a => Instruction(Opcode::ROL, AddressingMode::Accumulator),
            0x26 => Instruction(Opcode::ROL, AddressingMode::ZeroPage),
            0x36 => Instruction(Opcode::ROL, AddressingMode::ZeroPageX),
            0x2e => Instruction(Opcode::ROL, AddressingMode::Absolute),
            0x3e => Instruction(Opcode::ROL, AddressingMode::AbsoluteX),

            //ROR
            0x6a => Instruction(Opcode::ROR, AddressingMode::Accumulator),
            0x66 => Instruction(Opcode::ROR, AddressingMode::ZeroPage),
            0x76 => Instruction(Opcode::ROR, AddressingMode::ZeroPageX),
            0x6e => Instruction(Opcode::ROR, AddressingMode::Absolute),
            0x7e => Instruction(Opcode::ROR, AddressingMode::AbsoluteX),

            //Increment/Decrement

            //INC
            0xe6 => Instruction(Opcode::INC, AddressingMode::ZeroPage),
            0xf6 => Instruction(Opcode::INC, AddressingMode::ZeroPageX),
            0xee => Instruction(Opcode::INC, AddressingMode::Absolute),
            0xfe => Instruction(Opcode::INC, AddressingMode::AbsoluteX),

            //INX
            0xe8 => Instruction(Opcode::INX, AddressingMode::Implied),

            //INY
            0xc8 => Instruction(Opcode::INY, AddressingMode::Implied),

            //DEC
            0xc6 => Instruction(Opcode::DEC, AddressingMode::ZeroPage),
            0xd6 => Instruction(Opcode::DEC, AddressingMode::ZeroPageX),
            0xce => Instruction(Opcode::DEC, AddressingMode::Absolute),
            0xde => Instruction(Opcode::DEC, AddressingMode::AbsoluteX),

            //DEX
            0xca => Instruction(Opcode::DEX, AddressingMode::Implied),

            //DEY
            0x88 => Instruction(Opcode::DEY, AddressingMode::Implied),

            //Load/Store

            //LDA
            0xa9 => Instruction(Opcode::LDA, AddressingMode::Immediate),
            0xa5 => Instruction(Opcode::LDA, AddressingMode::ZeroPage),
            0xb5 => Instruction(Opcode::LDA, AddressingMode::ZeroPageX),
            0xad => Instruction(Opcode::LDA, AddressingMode::Absolute),
            0xbd => Instruction(Opcode::LDA, AddressingMode::AbsoluteX),
            0xb9 => Instruction(Opcode::LDA, AddressingMode::AbsoluteY),
            0xa1 => Instruction(Opcode::LDA, AddressingMode::IndirectX),
            0xb1 => Instruction(Opcode::LDA, AddressingMode::IndirectY),

            //LDX
            0xa2=> Instruction(Opcode::LDX, AddressingMode::Immediate),
            0xa6=> Instruction(Opcode::LDX, AddressingMode::ZeroPage),
            0xb6=> Instruction(Opcode::LDX, AddressingMode::ZeroPageX),
            0xae=> Instruction(Opcode::LDX, AddressingMode::Absolute),
            0xbe=> Instruction(Opcode::LDX, AddressingMode::AbsoluteY),

            //LDY
            0xa0 => Instruction(Opcode::LDY, AddressingMode::Immediate),
            0xa4 => Instruction(Opcode::LDY, AddressingMode::ZeroPage),
            0xb4 => Instruction(Opcode::LDY, AddressingMode::ZeroPageY),
            0xac => Instruction(Opcode::LDY, AddressingMode::Absolute),
            0xbc => Instruction(Opcode::LDY, AddressingMode::AbsoluteX),

            //STA
            0x85 => Instruction(Opcode::STA, AddressingMode::ZeroPage),
            0x95 => Instruction(Opcode::STA, AddressingMode::ZeroPageX),
            0x8d => Instruction(Opcode::STA, AddressingMode::Absolute),
            0x9d => Instruction(Opcode::STA, AddressingMode::AbsoluteX),
            0x99 => Instruction(Opcode::STA, AddressingMode::AbsoluteY),
            0x81 => Instruction(Opcode::STA, AddressingMode::IndirectX),
            0x91 => Instruction(Opcode::STA, AddressingMode::IndirectY),

            //STX
            0x86 => Instruction(Opcode::STX, AddressingMode::ZeroPage),
            0x86 => Instruction(Opcode::STX, AddressingMode::ZeroPageY),
            0x86 => Instruction(Opcode::STX, AddressingMode::Absolute),

            //STY
            0x84 => Instruction(Opcode::STY, AddressingMode::ZeroPage),
            0x84 => Instruction(Opcode::STY, AddressingMode::ZeroPageX),
            0x84 => Instruction(Opcode::STY, AddressingMode::Absolute),

            //Flag set/clear
            0x38 => Instruction(Opcode::SEC, AddressingMode::Implied),
            0xf8 => Instruction(Opcode::SED, AddressingMode::Implied),
            0x78 => Instruction(Opcode::SEI, AddressingMode::Implied),
            0x18 => Instruction(Opcode::CLC, AddressingMode::Implied),
            0xd8 => Instruction(Opcode::CLD, AddressingMode::Implied),
            0x58 => Instruction(Opcode::CLI, AddressingMode::Implied),
            0xb8 => Instruction(Opcode::CLV, AddressingMode::Implied),

            //Compare

            //CMP
            0xc9 => Instruction(Opcode::CMP, AddressingMode::Immediate),
            0xc5 => Instruction(Opcode::CMP, AddressingMode::ZeroPage),
            0xd5 => Instruction(Opcode::CMP, AddressingMode::ZeroPageX),
            0xcd => Instruction(Opcode::CMP, AddressingMode::Absolute),
            0xdd => Instruction(Opcode::CMP, AddressingMode::AbsoluteX),
            0xd9 => Instruction(Opcode::CMP, AddressingMode::AbsoluteY),
            0xc1 => Instruction(Opcode::CMP, AddressingMode::IndirectX),
            0xd1 => Instruction(Opcode::CMP, AddressingMode::IndirectY),

            //CPX
            0xe0 => Instruction(Opcode::CPX, AddressingMode::Immediate),
            0xe4 => Instruction(Opcode::CPX, AddressingMode::ZeroPage),
            0xec => Instruction(Opcode::CPX, AddressingMode::Absolute),

            //CPY
            0xc0 => Instruction(Opcode::CPY, AddressingMode::Immediate),
            0xc4 => Instruction(Opcode::CPY, AddressingMode::ZeroPage),
            0xcc => Instruction(Opcode::CPY, AddressingMode::Absolute),

            //Jmp/Ret

            //JMP
            0x4c => Instruction(Opcode::JMP, AddressingMode::Absolute),
            0x6c => Instruction(Opcode::JMP, AddressingMode::Indirect),

            //JSR
            0x20 => Instruction(Opcode::JSR, AddressingMode::Absolute),

            //RTI
            0x40 => Instruction(Opcode::RTI, AddressingMode::Implied),

            //RTS
            0x60 => Instruction(Opcode::RTS, AddressingMode::Implied),

            //Branch
            0x90 => Instruction(Opcode::BCC, AddressingMode::Relative),
            0xb0 => Instruction(Opcode::BCS, AddressingMode::Relative),
            0xf0 => Instruction(Opcode::BEQ, AddressingMode::Relative),
            0xd0 => Instruction(Opcode::BNE, AddressingMode::Relative),
            0x30 => Instruction(Opcode::BMI, AddressingMode::Relative),
            0x10 => Instruction(Opcode::BPL, AddressingMode::Relative),
            0x50 => Instruction(Opcode::BVC, AddressingMode::Relative),
            0x70 => Instruction(Opcode::BVS, AddressingMode::Relative),

            //Push and Pop
            0x48 => Instruction(Opcode::PHA, AddressingMode::Implied),
            0x08 => Instruction(Opcode::PHP, AddressingMode::Implied),
            0x68 => Instruction(Opcode::PLA, AddressingMode::Implied),
            0x28 => Instruction(Opcode::PLP, AddressingMode::Implied),

            //Transfer
            0xaa => Instruction(Opcode::TAX, AddressingMode::Implied),
            0xa8 => Instruction(Opcode::TAY, AddressingMode::Implied),
            0xBa => Instruction(Opcode::TSX, AddressingMode::Implied),
            0x8a => Instruction(Opcode::TXA, AddressingMode::Implied),
            0x9a => Instruction(Opcode::TXS, AddressingMode::Implied),
            0x98 => Instruction(Opcode::TYA, AddressingMode::Implied),

            //Misc

            //BRK
            0x00 => Instruction(Opcode::BRK, AddressingMode::Implied),

            //BIT
            0x24 => Instruction(Opcode::BIT, AddressingMode::ZeroPage),
            0x2c => Instruction(Opcode::BIT, AddressingMode::Absolute),
            //Unofficial
            0xea => Instruction(Opcode::NOP, AddressingMode::Implied),
            0x4b => Instruction(Opcode::ALR, AddressingMode::Immediate),
            0x0b => Instruction(Opcode::ANC, AddressingMode::Immediate),
            0x6b => Instruction(Opcode::ARR, AddressingMode::Immediate),
            0xcb => Instruction(Opcode::AXS, AddressingMode::Immediate),

            0xa3 => Instruction(Opcode::LAX, AddressingMode::IndirectX),
            0xa7 => Instruction(Opcode::LAX, AddressingMode::ZeroPage),
            0xaf => Instruction(Opcode::LAX, AddressingMode::Absolute),
            0xb3 => Instruction(Opcode::LAX, AddressingMode::IndirectY),
            0xb7 => Instruction(Opcode::LAX, AddressingMode::ZeroPageY),
            0xbf => Instruction(Opcode::LAX, AddressingMode::AbsoluteY),

            0x83 => Instruction(Opcode::SAX, AddressingMode::IndirectX),
            0x87 => Instruction(Opcode::SAX, AddressingMode::ZeroPage),
            0x8f => Instruction(Opcode::SAX, AddressingMode::Absolute),
            0x97 => Instruction(Opcode::SAX, AddressingMode::ZeroPageY),

            0xc3 => Instruction(Opcode::DCP, AddressingMode::IndirectX),
            0xc7 => Instruction(Opcode::DCP, AddressingMode::ZeroPage),
            0xcf => Instruction(Opcode::DCP, AddressingMode::Absolute),
            0xd3 => Instruction(Opcode::DCP, AddressingMode::IndirectY),
            0xd7 => Instruction(Opcode::DCP, AddressingMode::ZeroPageX),
            0xdb => Instruction(Opcode::DCP, AddressingMode::AbsoluteY),
            0xdf => Instruction(Opcode::DCP, AddressingMode::AbsoluteX),

            0xe3 => Instruction(Opcode::ISC, AddressingMode::IndirectX),
            0xe7 => Instruction(Opcode::ISC, AddressingMode::ZeroPage),
            0xef => Instruction(Opcode::ISC, AddressingMode::Absolute),
            0xf3 => Instruction(Opcode::ISC, AddressingMode::IndirectY),
            0xf7 => Instruction(Opcode::ISC, AddressingMode::ZeroPageX),
            0xfb => Instruction(Opcode::ISC, AddressingMode::AbsoluteY),
            0xff => Instruction(Opcode::ISC, AddressingMode::AbsoluteX),

            0x23 => Instruction(Opcode::RLA, AddressingMode::IndirectX),
            0x27 => Instruction(Opcode::RLA, AddressingMode::ZeroPage),
            0x2f => Instruction(Opcode::RLA, AddressingMode::Absolute),
            0x33 => Instruction(Opcode::RLA, AddressingMode::IndirectY),
            0x37 => Instruction(Opcode::RLA, AddressingMode::ZeroPageX),
            0x3b => Instruction(Opcode::RLA, AddressingMode::AbsoluteY),
            0x3f => Instruction(Opcode::RLA, AddressingMode::AbsoluteX),

            0x63 => Instruction(Opcode::RRA, AddressingMode::IndirectX),
            0x67 => Instruction(Opcode::RRA, AddressingMode::ZeroPage),
            0x6f => Instruction(Opcode::RRA, AddressingMode::Absolute),
            0x73 => Instruction(Opcode::RRA, AddressingMode::IndirectY),
            0x77 => Instruction(Opcode::RRA, AddressingMode::ZeroPageX),
            0x7b => Instruction(Opcode::RRA, AddressingMode::AbsoluteY),
            0x7f => Instruction(Opcode::RRA, AddressingMode::AbsoluteX),

            0x03 => Instruction(Opcode::SLO, AddressingMode::IndirectX),
            0x07 => Instruction(Opcode::SLO, AddressingMode::ZeroPage),
            0x0f => Instruction(Opcode::SLO, AddressingMode::Absolute),
            0x13 => Instruction(Opcode::SLO, AddressingMode::IndirectY),
            0x17 => Instruction(Opcode::SLO, AddressingMode::ZeroPageX),
            0x1b => Instruction(Opcode::SLO, AddressingMode::AbsoluteY),
            0x1f => Instruction(Opcode::SLO, AddressingMode::AbsoluteX),

            0x43 => Instruction(Opcode::SRE, AddressingMode::IndirectX),
            0x47 => Instruction(Opcode::SRE, AddressingMode::ZeroPage),
            0x4f => Instruction(Opcode::SRE, AddressingMode::Absolute),
            0x53 => Instruction(Opcode::SRE, AddressingMode::IndirectY),
            0x57 => Instruction(Opcode::SRE, AddressingMode::ZeroPageX),
            0x5b => Instruction(Opcode::SRE, AddressingMode::AbsoluteY),
            0x5f => Instruction(Opcode::SRE, AddressingMode::AbsoluteX),

            0x80 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0x82 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0x89 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0xc2 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0xe2 => Instruction(Opcode::SKB, AddressingMode::Immediate),

            0x0c => Instruction(Opcode::IGN, AddressingMode::Absolute),

            0x1c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0x3c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0x5c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0x7c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0xdc => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0xfc => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),

            0x04 => Instruction(Opcode::IGN, AddressingMode::ZeroPage),
            0x44 => Instruction(Opcode::IGN, AddressingMode::ZeroPage),
            0x64 => Instruction(Opcode::IGN, AddressingMode::ZeroPage),

            0x14 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0x34 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0x54 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0x74 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0xd4 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0xf4 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),

            _ =>  {
                log("Unimplemented opcode:");
                log(&code.to_string());
                Instruction(Opcode::NOP, AddressingMode::Implied)
            },
        }
    }
}