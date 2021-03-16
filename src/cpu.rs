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
    pub memory: [u8; 4096],
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
    pub fn new(memory: &[u8; 4096]) -> Cpu{
        Cpu{
            memory: *memory,
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
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            //Jmp/Ret
            0x4c => {
                log("Got jump");
                Instruction(Opcode::JMP, AddressingMode::Absolute)
            },
            //Misc
            0x00 =>{
                log("Got brk");
                Instruction(Opcode::BRK, AddressingMode::Implied)
            },
            _ =>  {
              //  log("Unimplemented opcode:");
              //  log(&code.to_string());
                Instruction(Opcode::ADC, AddressingMode::Immediate)
            },
        }
    }
}

impl Cpu {
    fn read_carry_flag(&self) -> bool{
        (self.p & 0x01u8) == 0x01u8
    }
    pub fn write_break_flag(&mut self, active:bool){
        if active{
        self.p = self.p | 0x10u8;
        }else{
            self.p = self.p & (!0x10u8);
        }
    }
    pub fn write_interrupt_flag(&mut self, active:bool){
        if active{
            self.p = self.p | 0x04u8;
        }else{
            self.p = self.p & (!0x04u8);
        }
    }
    pub fn read_interrupt_flag(&self)->bool{
        (self.p & 0x4u8) == 0x04u8
    }
    pub fn stack_push(&mut self, system: &mut System, data: u8){
        system.write_u8(self.s, data, false);
        log("Writing datas");
        self.s = self.s - 1;
    }
    pub fn interrupt(&mut self, system: &mut System, irq : Interrupt){
        let is_nested = self.read_interrupt_flag();

        match irq{
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
    fn fetch_operand(&mut self, system:  &mut System, mode: AddressingMode) ->Operand{
        match mode{
            
            AddressingMode::Implied => Operand(0, 0),
            _ => {
                //log("unmatched operand: ");
                Operand(0,0)}
                ,
        }
    }
    fn fetch_args(&mut self, system: &mut System, mode: AddressingMode) ->(Operand, u8){
        match mode{
            AddressingMode::Implied =>(self.fetch_operand(system, mode), 0),
            _ => (self.fetch_operand(system, mode), 0)
        }
    
    }
    pub fn step(&mut self, system : &mut System) -> u8{
        let inst_pc = self.pc;
        let inst_code = self.fetch8(system);
      //  log(&inst_code.to_string());
        let Instruction(opcode, mode) = Instruction::from(inst_code);
        
        match opcode{
            Opcode::ADC => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let t = u16::from(self.a) + u16::from(arg) + (if self.read_carry_flag() { 1 } else { 0 });
                let result = (t & 0xff) as u8;
                1 + cyc
            },
            Opcode::JMP =>{
                log("JUMP");
                69
            },
            Opcode::BRK =>{
                log("BORK");
                self.write_break_flag(true);
                self.interrupt(system, Interrupt::BRK);
                7
            },
            _ =>{
                log("Could not match opcode");
                0
            }
        }
    }
}
