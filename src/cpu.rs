use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
use crate::system;

use super::system::*;
pub const CPU_FREQ: u32 = 1790000;
pub const NMI_READ_LOWER: u16 = 0xfffa;
pub const NMI_READ_UPPER: u16 = 0xfffb;
pub const RESET_READ_LOWER: u16 = 0xfffc;
pub const RESET_READ_UPPER: u16 = 0xfffd;
pub const IRQ_READ_LOWER: u16 = 0xfffe;
pub const IRQ_READ_UPPER: u16 = 0xffff;
pub const BRK_READ_LOWER: u16 = 0xfffe;
pub const BRK_READ_UPPER: u16 = 0xffff;
#[derive(Debug, Clone)]
pub struct Cpu{
    pub pc : u16, //2-byte program counter
    x : u8,
    y : u8, //X and Y are index registers
    a  : u8, //Accumulator
    s : u16, //Stack Pointer
    p : u8, //Status Register
    pub data : u8, //Last data read, for debug
}

#[derive(PartialEq, Eq)]
pub enum Interrupt {
    NMI,
    RESET,
    IRQ,
    BRK,
}

impl Cpu{
    pub fn increment(&mut self, incr:u16){
        self.pc = self.pc + incr;
    }
    
}
//Where we eventually have all of our opcodes enumerated, this will be a fairly large enum
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Opcode{
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
}

impl Cpu{
    pub fn new() -> Cpu{
        Cpu{
            pc : 0,
            x : 0,
            y : 0,
            a : 0,
            s : 0,
            p : 0,
            data : 0,
        }
    }
}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum AddressingMode {
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
#[derive(Clone, Copy)]
struct Operand(u16, u8);
#[derive(Clone, Copy, Debug)]
struct Instruction(Opcode, AddressingMode);


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

            0xea => Instruction(Opcode::NOP, AddressingMode::Implied),

            _ =>  {
                log("Unimplemented opcode:");
                log(&code.to_string());
                Instruction(Opcode::ADC, AddressingMode::Immediate)
            },
        }
    }
}

impl Cpu {

    pub fn write_negative_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x80u8;
        }else{
            self.p = self.p & (!0x80u8);
        }
    }

    pub fn write_overflow_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x40u8;
        }else{
            self.p = self.p & (!0x40u8);
        }
    }

    pub fn write_break_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x10u8;
        }else{
            self.p = self.p & (!0x10u8);
        }
    }

    pub fn write_reserved_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x20u8;
        }else{
            self.p = self.p & (!0x20u8);
        }
    }

    pub fn write_decimal_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x08u8;
        }else{
            self.p = self.p & (!0x08u8);
        }
    }

    pub fn write_interrupt_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x04u8;
        }else{
            self.p = self.p & (!0x04u8);
        }
    }
    
    pub fn write_zero_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x02u8;
        }else{
            self.p = self.p & (!0x02u8);
        }
    }

    pub fn write_carry_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x01u8;
        }else{
            self.p = self.p & (!0x01u8);
        }
    }
    pub fn read_negative_flag(&self) -> bool {
        (self.p & 0x80u8) == 0x80u8
    }
    pub fn read_overflow_flag(&self) -> bool {
        (self.p & 0x40u8) == 0x40u8
    }
    pub fn read_reserved_flag(&self) -> bool {
        (self.p & 0x20u8) == 0x20u8
    }
    pub fn read_break_flag(&self) -> bool {
        (self.p & 0x10u8) == 0x10u8
    }
    pub fn read_decimal_flag(&self) -> bool {
        (self.p & 0x08u8) == 0x08u8
    }
    pub fn read_zero_flag(&self) -> bool {
        (self.p & 0x02u8) == 0x02u8
    }
    pub fn read_carry_flag(&self) -> bool{
        (self.p & 0x01u8) == 0x01u8
    }
    pub fn read_interrupt_flag(&self)->bool{
        (self.p & 0x4u8) == 0x04u8
    }

    pub fn stack_push(&mut self, system: &mut System, data: u8){
        system.write_u8(self.s, data, false);
        self.s = self.s - 1;
    }
    pub fn stack_pop(&mut self, system: &mut System) -> u8 {
        self.s = self.s + 1;
        system.read_u8(self.s, false)
    }
    pub fn interrupt(&mut self, system: &mut System, irq : Interrupt){
        let is_nested = self.read_interrupt_flag();

        match irq{
            Interrupt::NMI =>{
            self.write_break_flag(false);
            self.stack_push(system, (self.pc >> 8) as u8);
            self.stack_push(system, (self.pc & 0xff) as u8);
            self.stack_push(system, self.p);
            self.write_interrupt_flag(true);
            },
            Interrupt::RESET => {
                self.write_interrupt_flag(true)
            },
            Interrupt::IRQ => {
                self.write_break_flag(false);
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true)
            },
            Interrupt::BRK =>{
                self.write_break_flag(true);
                self.pc = self.pc + 1;

                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);

            },
            _ => {
                log("Unhandled IRQ");
            }
        }

        let lower = match irq{
            Interrupt::BRK => BRK_READ_LOWER,
            Interrupt::NMI => NMI_READ_LOWER,
            Interrupt::IRQ => IRQ_READ_LOWER,
            Interrupt::RESET => RESET_READ_LOWER
        };
        let upper = match irq{
            Interrupt::BRK => BRK_READ_UPPER,
            Interrupt::NMI => NMI_READ_UPPER,
            Interrupt::IRQ => IRQ_READ_UPPER,
            Interrupt::RESET => RESET_READ_UPPER
        };
        
        let lower_d = system.read_u8(lower, false);
        let upper_d = system.read_u8(upper, false);
        self.pc = (lower_d as u16) | ((upper_d as u16) << 8);

    }
    fn fetch8(&mut self, sys: &mut System) -> u8{
        let data = sys.read_u8(self.pc, false);
        self.pc = self.pc + 1;
        data
    }
    fn fetch16(&mut self, sys: &mut System) ->u16{
        let lower = self.fetch8(sys);
        let upper = self.fetch8(sys);
        let data = u16::from(lower) | (u16::from(upper) << 8);
        data
    }
    fn fetch_operand(&mut self, system:  &mut System, mode: AddressingMode) ->Operand{
        match mode{
            
            AddressingMode::Implied => Operand(0, 0),
            AddressingMode::Accumulator => Operand(0, 1),
            AddressingMode::Immediate => Operand(u16::from(self.fetch8(system)), 1),
            AddressingMode::Absolute => Operand(self.fetch16(system), 3),
            AddressingMode::ZeroPage => Operand(u16::from(self.fetch8(system)),2),
            AddressingMode::ZeroPageX => Operand(u16::from(self.fetch8(system).wrapping_add(self.x)), 3),
            AddressingMode::ZeroPageY => Operand(u16::from(self.fetch8(system).wrapping_add(self.y)), 3),
            AddressingMode::AbsoluteX => {
                let data = self.fetch16(system).wrapping_add(u16::from(self.x));
                let add_cyc = if(data & 0xff00u16) != (data.wrapping_add(u16::from(self.x)) & 0xff00u16) {
                    1
                } else{
                    0
                };
                Operand(data, 3 + add_cyc)
            },
            AddressingMode::AbsoluteY => {
                let data = self.fetch16(system).wrapping_add(u16::from(self.y));
                let add_cyc = if(data & 0xff00u16) != (data.wrapping_add(u16::from(self.y)) & 0xff00u16) {
                    1
                } else{
                    0
                };
                Operand(data, 3 + add_cyc)
            },

            AddressingMode::Relative => {
                let src_addr = self.fetch8(system);
                let signed_d = ((src_addr as i8) as i32) + (self.pc as i32);
                let data = signed_d as u16;
                let add_cyc = if(data & 0xff00u16) != (self.pc & 0xff00u16){
                    1
                }else{
                    0
                };
                Operand(data, 1 + add_cyc)
            },

            AddressingMode::Indirect => {
                let src_addr_lower = self.fetch8(system);
                let src_addr_upper = self.fetch8(system);

                let dst_addr_lower = u16::from(src_addr_lower) | (u16::from(src_addr_upper) << 8);
                let dst_addr_upper = u16::from(src_addr_lower.wrapping_add(1)) | (u16::from(src_addr_upper) << 8);
                let dst_data_lower = u16::from(system.read_u8(dst_addr_lower, false));
                let dst_data_upper = u16::from(system.read_u8(dst_addr_upper, false));

                let data = dst_data_lower | (dst_data_upper << 8);

                Operand(data, 5)
            },

            AddressingMode::IndirectX => {
                let src_addr = self.fetch8(system);
                let dst_addr = src_addr.wrapping_add(self.x);

                let data_lower = u16::from(system.read_u8(u16::from(dst_addr), false));
                let data_upper = u16::from(system.read_u8(u16::from(dst_addr.wrapping_add(1)), false));

                let data = data_lower | (data_upper << 8);

                Operand(data, 5)
                
            }
            AddressingMode::IndirectY => {
                let src_addr = self.fetch8(system);

                let data_lower = u16::from(system.read_u8(u16::from(src_addr), false));
                let data_upper =
                    u16::from(system.read_u8(u16::from(src_addr.wrapping_add(1)), false));

                let base_data = data_lower | (data_upper << 8);
                let data = base_data.wrapping_add(u16::from(self.y));
                let additional_cyc = if (base_data & 0xff00u16) != (data & 0xff00u16) {
                    1
                } else {
                    0
                };

                Operand(data, 4 + additional_cyc)
                
            }

            _ => {
                //log("unmatched operand: ");
                Operand(0,0)}
                ,
        }
    }
    fn fetch_args(&mut self, system: &mut System, mode: AddressingMode) ->(Operand, u8){
        match mode{
            AddressingMode::Implied =>(self.fetch_operand(system, mode), 0),
            AddressingMode::Accumulator => (self.fetch_operand(system, mode), self.a),
            AddressingMode::Immediate => {
                let Operand(data, cyc) = self.fetch_operand(system, mode);
                (Operand(data, cyc), data as u8)
            }
            _ => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                let data = system.read_u8(addr, false);
                (Operand(addr, cyc), data)
            }
        }
    
    }
    pub fn step(&mut self, system : &mut System) -> u8{
        let inst_pc = self.pc;
        let inst_code = self.fetch8(system);
      //  log(&inst_code.to_string());
        let Instruction(opcode, mode) = Instruction::from(inst_code);
        
        match opcode{
            Opcode::ADC => {
                log("ADC");
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let tmp = u16::from(self.a) + u16::from(arg) + (if self.read_carry_flag() { 1 } else { 0 } );
                let result = (tmp & 0xff) as u8;

                let carry_flag    = tmp > 0x00ffu16;
                let zero_flag     = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                let overflow_flag = ((self.a ^ result) & (arg ^ result) & 0x80) == 0x80;

                self.write_carry_flag(carry_flag);
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.write_overflow_flag(overflow_flag);
                self.a = result;
                1 + cyc
            },
            Opcode::SBC => {
                log("SBC");
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);
                let (data, carry1) = self.a.overflowing_sub(arg);
                let (result, carry2) = data.overflowing_sub(if self.read_carry_flag() {0} else {1});

                let carry_flag = !(carry1 || carry2);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                let overflow_flag = (((self.a ^ arg) & 0x80) == 0x80) && (((self.a ^ result) & 0x80) == 0x80);
                self.write_carry_flag(carry_flag);
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.write_overflow_flag(overflow_flag);
                self.a = result;
                1 + cyc
            },
            Opcode::AND => {
                log("AND");
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a & arg;
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.a = result;
                1 + cyc
            },
            Opcode::EOR => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a ^ arg;

                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);

                1 + cyc
            },
            Opcode::ORA => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a | arg;

                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);

                self.a = result;
                1 + cyc
            },
            Opcode::ASL =>{
                let (Operand(addr,cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shl(1);

                let carry_flag = (arg & 0x80) == 0x80;
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.write_carry_flag(carry_flag);
                if mode == AddressingMode::Accumulator{
                    self.a = result;
                    1 + cyc
                }else{
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            Opcode::LSR => {
                let (Operand(addr,cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shr(1);

                let carry_flag = (arg & 0x01) == 0x01;
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.write_carry_flag(carry_flag);
                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else{
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            Opcode::ROL => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01} else {0x00});
                let carry_flag = (arg & 0x80) == 0x80;
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.write_carry_flag(carry_flag);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else{
                    system.write_u8(addr, result, false);
                    3 + cyc
                }

            },
            Opcode::ROR => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80} else {0x00});
                let carry_flag = (arg & 0x01) == 0x01;
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.write_carry_flag(carry_flag);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else{
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            Opcode::INC => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_add(1);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                system.write_u8(addr, result, false);
                3 + cyc
            },
            Opcode::INX => {
                let result = self.x.wrapping_add(1);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.x = result;
                2
            },
            Opcode::INY => {
                let result = self.y.wrapping_add(1);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.y = result;
                2
            },
            Opcode::DEC => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);
                let result = self.x.wrapping_sub(1);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                system.write_u8(addr, result, false);
                3 + cyc
            },
            Opcode::DEX => {
                let result = self.x.wrapping_sub(1);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.x = result;
                2
            },
            Opcode::DEY => {
                let result = self.y.wrapping_sub(1);
                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.y = result;
                2
            },
            Opcode::LDA => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);
                let zero_flag = arg == 0;
                let negative_flag = (arg & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.a = arg;
                1 + cyc
            },
            Opcode::LDX => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);
                let zero_flag = arg == 0;
                let negative_flag = (arg & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.x = arg;
                1 + cyc
                
            },
            Opcode::LDY =>{
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);
                let zero_flag = arg == 0;
                let negative_flag = (arg & 0x80) == 0x80;
                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.y = arg;
                1 + cyc
            },
            Opcode::STA => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);

                system.write_u8(addr,self.a, false);
                1 + cyc
            },
            Opcode::STX => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);

                system.write_u8(addr,self.x, false);
                1 + cyc
            },
            Opcode::STY => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);

                system.write_u8(addr,self.y, false);
                1 + cyc
            },
            Opcode::SEC => {
                self.write_carry_flag(true);
                2
            },
            Opcode::SED => {
                self.write_decimal_flag(true);
                2
            },
            Opcode::SEI => {
                self.write_interrupt_flag(true);
                2
            },
            Opcode::CLC => {
                self.write_carry_flag(false);
                2
            },
            Opcode::CLD => {
                self.write_decimal_flag(false);
                2
            },
            Opcode::CLI => {
                self.write_interrupt_flag(false);
                2
            },
            Opcode::CLV => {
                self.write_overflow_flag(false);
                2
            },
            Opcode::CMP => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (result, _) = self.a.overflowing_sub(arg);
                let zero_flag = result == 0;
                let carry_flag = self.a >= arg;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_carry_flag(carry_flag);
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                1 + cyc
            },
            Opcode::CPX => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (result, _) = self.x.overflowing_sub(arg);
                let zero_flag = result == 0;
                let carry_flag = self.x >= arg;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_carry_flag(carry_flag);
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                1 + cyc
            },
            Opcode::CPY => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (result, _) = self.y.overflowing_sub(arg);
                let zero_flag = result == 0;
                let carry_flag = self.y >= arg;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_carry_flag(carry_flag);
                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                1 + cyc
            },
            Opcode::JMP => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                self.pc = addr;
                cyc
            },
            Opcode::JSR => {
                let Operand(addr, _) = self.fetch_operand(system, mode);

                let opcode_addr = inst_pc;

                let ret_addr = opcode_addr + 2;
                self.stack_push(system, (ret_addr >> 8) as u8);
                self.stack_push(system, (ret_addr & 0xff) as u8);
                self.pc = addr;
                6
            },
            Opcode::RTI => {
                self.p = self.stack_pop(system);

                let pc_lower = self.stack_pop(system);
                let pc_upper = self.stack_pop(system);

                self.pc = ((pc_upper as u16) << 8) | (pc_lower as u16);
                6
            },
            Opcode::RTS =>{
                let pc_lower = self.stack_pop(system);
                let pc_upper = self.stack_pop(system);
                self.pc = (((pc_upper as u16) << 8) | (pc_lower as u16)) + 1;
                6
            },
            Opcode::BCC => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_carry_flag(){
                    self.pc = addr;
                    2 + cyc
                }else{
                    1 + cyc
                }
            },
            Opcode::BCS => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_carry_flag() {
                    self.pc = addr;
                    2 + cyc
                } else {
                    1 + cyc
                }
            },
            Opcode::BEQ => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_zero_flag() {
                    self.pc = addr;
                    2 + cyc
                } else {
                    cyc + 1
                }
            },
            Opcode::BNE => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_zero_flag() {
                    self.pc = addr;
                    2 + cyc
                } else {
                    1 + cyc
                }
            },
            Opcode::BMI => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_negative_flag() {
                    self.pc = addr;
                    2 + cyc 
                }else{
                    1 + cyc
                }
            },
            Opcode::BPL => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_negative_flag() {
                    self.pc = addr;
                    2 + cyc 
                }else{
                    1 + cyc
                }
            },
            Opcode::BVC => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_overflow_flag() {
                    self.pc = addr;
                    2 + cyc 
                }else{
                    1 + cyc
                }
            },
            Opcode::BVS => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_overflow_flag() {
                    self.pc = addr;
                    2 + cyc 
                }else{
                    1 + cyc
                }
            },
            Opcode::PHA => {
                self.stack_push(system, self.a);
                3
            },
            Opcode::PHP => {
                self.stack_push(system, self.p);
                3
            },
            Opcode::PLA => {
                let result = self.stack_pop(system);

                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.a = result;
                4
            },
            Opcode::PLP => {
                self.p = self.stack_pop(system);
                4
            },
            Opcode::TAX => {
                let zero_flag = self.a == 0;
                let negative_flag = (self.a & 0x80) == 0x80;

                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);

                self.x = self.a;
                2
            },
            Opcode::TAY => {
                let zero_flag = self.a == 0;
                let negative_flag = (self.a & 0x80) == 0x80;

                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);

                self.y = self.a;
                2
            },
            Opcode::TSX => {
                let result = (self.s & 0xff) as u8;

                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.x = result;

                2
            },
            Opcode::TXA => {
                let zero_flag = self.y == 0;
                let negative_flag = (self.y & 0x80) == 0x80;

                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.a = self.y;
                2
            },
            Opcode::TXS =>{
                self.s = (self.x as u16) | 0x0100u16;
                2
            },
            Opcode::TYA =>{
                let zero_flag = self.y == 0;
                let negative_flag = (self.y & 0x80) == 0x80;

                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.a = self.y;
                2
            }
            Opcode::BRK =>{
                log("BRK");
                self.write_break_flag(true);
                self.interrupt(system, Interrupt::BRK);
                7
            },
            Opcode::BIT => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                let arg = system.read_u8(addr, true);

                let negative_flag = (arg & 0x80) == 0x80;
                let overflow_flag = (arg & 0x40) == 0x40;
                let zero_flag = (self.a & arg) == 0x00;

                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.write_overflow_flag(overflow_flag);
                2 + cyc
            },
            Opcode::NOP =>{
                2
            },
            _ =>{
                log("Could not match opcode");
                0
            }
        }
    }
}
