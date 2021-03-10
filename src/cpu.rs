use super::system::*;
#[derive(Debug, Clone)]
pub struct Cpu{
    pub memory: [u8; 4096],
    pub PC : u16, //2-byte program counter
    X : u8,
    Y : u8, //X and Y are index registers
    A : u8, //Accumulator
    S : u8, //Stack Pointer
    P : u8, //Status Register
}
//Where we eventually have all of our opcodes enumerated, this will be a fairly large enum
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Opcode{
    ADC,
}

impl Cpu{
    pub fn new(memory: &[u8; 4096]) -> Cpu{
        Cpu{
            memory: *memory,
            PC : 0,
            X : 0,
            Y : 0,
            A : 0,
            S : 0,
            P : 0,
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
#[derive(Clone, Copy)]
struct Instruction(Opcode, AddressingMode);


impl Instruction {
    pub fn ex(code : u8) -> Instruction {
        match code {
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            _ => panic!("Invalid {:08x}", code),
        }
    }
}

impl Cpu {
    fn fetch8(&mut self, sys: &mut System) -> u8{
        let data = sys.read_u8(self.PC, false);
        self.PC = self.PC + 1;
        data
    }
}
