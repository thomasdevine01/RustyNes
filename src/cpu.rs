use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
use super::system::*;

#[derive(Debug, Clone)]
pub struct Cpu{
    pub memory: [u8; 4096],
    pub pc : u16, //2-byte program counter
    x : u8,
    y : u8, //X and Y are index registers
    a  : u8, //Accumulator
    s : u8, //Stack Pointer
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
        match code {
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            _ =>  Instruction(Opcode::ADC, AddressingMode::Immediate),
        }
    }
}

impl Cpu {
    fn read_carry_flag(&self) -> bool{
        (self.p & 0x01u8) == 0x01u8
    }
    fn fetch8(&mut self, sys: &mut System) -> u8{
        let data = sys.read_u8(self.pc, false);
        self.pc = self.pc + 1;
        data
    }
    fn fetch_operand(&mut self, system:  &mut System, mode: AddressingMode) ->Operand{
        match mode{
            AddressingMode::Implied => Operand(0, 0),
            _ => Operand(0,0),
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
        
        let Instruction(opcode, mode) = Instruction::from(inst_code);
        
        match opcode{
            Opcode::ADC => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let t = u16::from(self.a) + u16::from(arg) + (if self.read_carry_flag() { 1 } else { 0 });
                let result = (t & 0xff) as u8;
                1 + cyc
            }
            _ =>(0)
        }
    }
}
