use wasm_bindgen::prelude::*;


#[cfg(feature = "unsafe-opt")]
#[allow(unused_macros)]
macro_rules! arr_read {
    ($arr:expr, $index:expr) => {
        unsafe { *$arr.get_unchecked($index) }
    };
}

#[cfg(feature = "unsafe-opt")]
#[allow(unused_macros)]
macro_rules! arr_write {
    ($arr:expr, $index:expr, $data:expr) => {
        unsafe { *$arr.get_unchecked_mut($index) = $data }
    };
}

#[cfg(not(feature = "unsafe-opt"))]
#[allow(unused_macros)]
macro_rules! arr_read {
    ($arr:expr, $index:expr) => {
        $arr[$index]
    };
}

#[cfg(not(feature = "unsafe-opt"))]
#[allow(unused_macros)]
macro_rules! arr_write {
    ($arr:expr, $index:expr, $data:expr) => {
        $arr[$index] = $data
    };
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


use super::system::*;
use super::instruction::*;

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
   pub x : u8,
    pub y : u8, //X and Y are index registers
    pub a  : u8, //Accumulator
    pub s : u16, //Stack Pointer
    pub p : u8, //Status Register

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
    pub fn reset(&mut self){
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0;
        self.s = 0x01fd;
        self.p = 0x34;
    }
    pub fn regstat(&self, reg:u8) -> u8{
        match reg {
            0 => self.x,
            1 => self.y,
            2 => self.a,
            _ => 254,
        }
    }
    pub fn regstat16(&self, reg:u8) -> u16 {
        match reg {
            0 => self.pc,
            1 => self.s,
            _ => 404,
        }
    }
 
    
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

        }
    }
}



//Write flags, do the logic on them according to various spreadsheets

impl Cpu {

    pub fn write_negative_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x80u8;
        } else {
            self.p = self.p & (!0x80u8);
        }
    }
    pub fn write_overflow_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x40u8;
        } else {
            self.p = self.p & (!0x40u8);
        }
    }
    pub fn write_reserved_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x20u8;
        } else {
            self.p = self.p & (!0x20u8);
        }
    }
    pub fn write_break_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x10u8;
        } else {
            self.p = self.p & (!0x10u8);
        }
    }
    pub fn write_decimal_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x08u8;
        } else {
            self.p = self.p & (!0x08u8);
        }
    }
    pub fn write_interrupt_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x04u8;
        } else {
            self.p = self.p & (!0x04u8);
        }
    }
    pub fn write_zero_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x02u8;
        } else {
            self.p = self.p & (!0x02u8);
        }
    }
    pub fn write_carry_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x01u8;
        } else {
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
    pub fn read_interrupt_flag(&self) -> bool {
        (self.p & 0x04u8) == 0x04u8
    }
    pub fn read_zero_flag(&self) -> bool {
        (self.p & 0x02u8) == 0x02u8
    }
    pub fn read_carry_flag(&self) -> bool {
        (self.p & 0x01u8) == 0x01u8
    }
    //Write to the stack register
    pub fn stack_push(&mut self, system: &mut System, data: u8){
        system.write_u8(self.s, data, false);
        self.s = self.s - 1;
    }
    //Pop from stack register
    pub fn stack_pop(&mut self, system: &mut System) -> u8 {
        self.s = self.s + 1;
        system.read_u8(self.s, false)
    }
    //The 6502 has 4 interrupts, NMI, RESET, IRQ, and FLAG
    //They are pretty self-explanatory
    pub fn interrupt(&mut self, system: &mut System, irq : Interrupt){
        let is_nested = self.read_interrupt_flag();
        if is_nested && (irq == Interrupt::IRQ) || (irq == Interrupt::BRK) {
            return;
        }
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
    //Fetch 8 bytes from the bus, quite important this one
    fn fetch8(&mut self, sys: &mut System) -> u8{
        let data = sys.read_u8(self.pc, false);
        self.pc = self.pc + 1;
        data
    }
    //Fetch 16 bytes from the bus, not nearly as important
    fn fetch16(&mut self, sys: &mut System) ->u16{
        let lower = self.fetch8(sys);
        let upper = self.fetch8(sys);
        let data = u16::from(lower) | (u16::from(upper) << 8);
        data
    }
    //Decouple operands using addressing modes from instructions
    //Reducing the work needed to be done by me by many fold
    //We have 13 addressing modes, most of them are self-explanatory
    fn fetch_operand(&mut self, system: &mut System, mode: AddressingMode) -> Operand {
        match mode {
            //Means we already know where the data is
            AddressingMode::Implied => Operand(0, 0),
            //Use the acc register
            AddressingMode::Accumulator => Operand(0, 1),
            //Use the argument to jump
            AddressingMode::Immediate => Operand(u16::from(self.fetch8(system)), 1),
            //Use the argument to directly go to the address, as long as its less than 16 bytes worth of addresses away
            AddressingMode::Absolute => Operand(self.fetch16(system), 3),
            // Go from the first page of memory
            AddressingMode::ZeroPage => Operand(u16::from(self.fetch8(system)), 2),
            //Go from the zero page by the x register
            AddressingMode::ZeroPageX => {
                Operand(u16::from(self.fetch8(system).wrapping_add(self.x)), 3)
            }
            //Go from the zero page by the Y register
            AddressingMode::ZeroPageY => {
                Operand(u16::from(self.fetch8(system).wrapping_add(self.y)), 3)
            }
            //Same as absolute above but added to X reg
            AddressingMode::AbsoluteX => {
                let data = self.fetch16(system).wrapping_add(u16::from(self.x));
                let additional_cyc =
                    if (data & 0xff00u16) != (data.wrapping_add(u16::from(self.x)) & 0xff00u16) {
                        1
                    } else {
                        0
                    };
                Operand(data, 3 + additional_cyc)
            }
            //...You can guess
            AddressingMode::AbsoluteY => {
                let data = self.fetch16(system).wrapping_add(u16::from(self.y));
                let additional_cyc =
                    if (data & 0xff00u16) != (data.wrapping_add(u16::from(self.y)) & 0xff00u16) {
                        1
                    } else {
                        0
                    };
                Operand(data, 3 + additional_cyc)
            }
            //Move relative to the current PC, this is used by branches mostly
            AddressingMode::Relative => {
                let src_addr = self.fetch8(system);
                let signed_data = ((src_addr as i8) as i32) + (self.pc as i32); 
                debug_assert!(signed_data >= 0);
                debug_assert!(signed_data < 0x10000);

                let data = signed_data as u16;
                let additional_cyc = if (data & 0xff00u16) != (self.pc & 0xff00u16) {
                    1
                } else {
                    0
                };

                Operand(data, 1 + additional_cyc)
            }
            //Jump to the address pointed to by a 16 bit address in memories (yes, the 6502 has pointers)
            AddressingMode::Indirect => {
                let src_addr_lower = self.fetch8(system);
                let src_addr_upper = self.fetch8(system);

                let dst_addr_lower = u16::from(src_addr_lower) | (u16::from(src_addr_upper) << 8); 
                let dst_addr_upper =
                    u16::from(src_addr_lower.wrapping_add(1)) | (u16::from(src_addr_upper) << 8); 

                let dst_data_lower = u16::from(system.read_u8(dst_addr_lower, false));
                let dst_data_upper = u16::from(system.read_u8(dst_addr_upper, false));

                let data = dst_data_lower | (dst_data_upper << 8);

                Operand(data, 5)
            }
            AddressingMode::IndirectX => {
                let src_addr = self.fetch8(system);
                let dst_addr = src_addr.wrapping_add(self.x);

                let data_lower = u16::from(system.read_u8(u16::from(dst_addr), false));
                let data_upper =
                    u16::from(system.read_u8(u16::from(dst_addr.wrapping_add(1)), false));

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
        }
    }
    //Get the arguments for an operation based on addressing mode
    fn fetch_args(&mut self, system: &mut System, mode: AddressingMode) ->(Operand, u8){
        match mode{
            AddressingMode::Implied =>(self.fetch_operand(system, mode), 0),
            AddressingMode::Accumulator => (self.fetch_operand(system, mode), self.a),
            AddressingMode::Immediate => {
                let Operand(data, cyc) = self.fetch_operand(system, mode);
                debug_assert!(data < 0x100u16);
                (Operand(data, cyc), data as u8)
            }
            _ => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                let data = system.read_u8(addr, false);
                (Operand(addr, cyc), data)
            }
        }
    
    }
    //The meat of the CPU, this function is an OO abomination but without costly abstraction, this is really the easiest way
    //I do not have time to explain every operation here. Or any of them. Look them up. It's neat.
    pub fn step(&mut self, system : &mut System) -> u8{
        let inst_pc = self.pc;
        let inst_code = self.fetch8(system);
        
        let Instruction(opcode, mode) = Instruction::from(inst_code);
        
        match opcode{
            Opcode::ADC => {
             //   //log("ADC");
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
            Opcode::AND => {
              //  //log("AND");
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a & arg;
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
            
            Opcode::RTS =>{
                let pc_lower = self.stack_pop(system);
                let pc_upper = self.stack_pop(system);
                self.pc = (((pc_upper as u16) << 8) | (pc_lower as u16)) + 1;
                6
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

                let is_carry    = self.y >= arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                1 + cyc
            }
            Opcode::DEC => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);
                let result = arg.wrapping_sub(1);
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
            Opcode::SBC => {
                
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

            Opcode::EOR => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a ^ arg;

                let zero_flag = result == 0;
                let negative_flag = (result & 0x80) == 0x80;

                self.write_zero_flag(zero_flag);
                self.write_negative_flag(negative_flag);
                self.a = result;
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
                let zero_flag = self.x == 0;
                let negative_flag = (self.x & 0x80) == 0x80;

                self.write_negative_flag(negative_flag);
                self.write_zero_flag(zero_flag);
                self.a = self.x;
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
               // //log("BRK");
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
            Opcode::ALR => {
                
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let src = self.a & arg;
                let result = src.wrapping_shr(1);

                let is_carry    = (src    & 0x01) == 0x01;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                self.a = result;
                1 + cyc
            },
            Opcode::ANC => {
               
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a & arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_carry    = self.read_negative_flag();

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_carry_flag(is_carry);
                self.a = result;
                1 + cyc
            },
            Opcode::ARR => {
             
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let src = self.a & arg;
                let result = src.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_carry    = (result & 0x40) == 0x40;
                let is_overflow = ((result & 0x40) ^ ((result & 0x20) << 1)) == 0x40;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_carry_flag(is_carry);
                self.write_overflow_flag(is_overflow);

                self.a = result;
                1 + cyc
            },
            Opcode::AXS => {
               
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let src = self.a & arg;

                let (result, is_carry) = self.a.overflowing_sub(src);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = result;
                1 + cyc
            },
            Opcode::LAX => {
                
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let is_zero     = arg == 0;
                let is_negative = (arg & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = arg;
                self.x = arg;
                1 + cyc
            },
            Opcode::SAX => {
                
                let (Operand(addr, cyc), _arg) = self.fetch_args(system, mode);

                let result = self.a & self.x;

                system.write_u8(addr, result, false);
                1 + cyc
            },
            Opcode::DCP => {
                
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                
                let dec_result = arg.wrapping_sub(1);
                system.write_u8(addr, dec_result, false);

                
                let result = self.a.wrapping_sub(dec_result);

                let is_carry    = self.a >= dec_result;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                3 + cyc
            },
            Opcode::ISC => {
                
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                
                let inc_result = arg.wrapping_add(1);
                system.write_u8(addr, inc_result, false);

                
                let (data1, is_carry1) = self.a.overflowing_sub(inc_result);
                let (result, is_carry2) = data1.overflowing_sub(if self.read_carry_flag() { 0 } else { 1 } );

                let is_carry    = !(is_carry1 || is_carry2); 
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_overflow = (((self.a ^ inc_result) & 0x80) == 0x80) && (((self.a ^ result) & 0x80) == 0x80);

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result;
                1 + cyc
            },
            Opcode::RLA => {
                
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

               
                let result_rol = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

                let is_carry    = (arg & 0x80) == 0x80;
                self.write_carry_flag(is_carry);

                system.write_u8(addr, result_rol, false);

                
                let result_and = self.a & result_rol;

                let is_zero     = result_and == 0;
                let is_negative = (result_and & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                self.a = result_and;

                3 + cyc
            },
            Opcode::RRA => {
                
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                
                let result_ror = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

                let is_carry_ror    = (arg & 0x01) == 0x01;
                self.write_carry_flag(is_carry_ror);

                system.write_u8(addr, result_ror, false);

                
                let tmp = u16::from(self.a) + u16::from(result_ror) + (if self.read_carry_flag() { 1 } else { 0 } );
                let result_adc = (tmp & 0xff) as u8;

                let is_carry    = tmp > 0x00ffu16;
                let is_zero     = result_adc == 0;
                let is_negative = (result_adc & 0x80) == 0x80;
                let is_overflow = ((self.a ^ result_adc) & (result_ror ^ result_adc) & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result_adc;

                3 + cyc
            },
            Opcode::SLO => {
                
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                
                let result_asl = arg.wrapping_shl(1);

                let is_carry    = (arg & 0x80) == 0x80; 
                self.write_carry_flag(is_carry);

                system.write_u8(addr, result_asl, false);

                
                let result_ora = self.a | result_asl;

                let is_zero     = result_ora == 0;
                let is_negative = (result_ora & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result_ora;

                3 + cyc
            },
            Opcode::SRE => {
                
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                
                let result_lsr = arg.wrapping_shr(1);

                let is_carry    = (arg & 0x01) == 0x01;
                self.write_carry_flag(is_carry);

                system.write_u8(addr, result_lsr, false);

               
                let result_eor = self.a ^ result_lsr;

                let is_zero     = result_eor == 0;
                let is_negative = (result_eor & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result_eor;

                3 + cyc
            },
            Opcode::SKB => {
                
               
                let (Operand(_addr, cyc), _arg) = self.fetch_args(system, mode);

                1 + cyc
            },
            Opcode::IGN => {
               
                let (Operand(_addr, cyc), _arg) = self.fetch_args(system, mode);

                1 + cyc
            },
            Opcode::NOP =>{
                2
            },

        }
    }
}
