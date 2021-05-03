
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
/*
This is essentially the bus, and will become the core of the emulator, as it transmits data between each component
*/
pub const WRAM_SIZE: usize = 0x0800;
pub const PPU_REG_SIZE: usize = 0x0008;
pub const APU_IO_REG_SIZE: usize = 0x0018;
pub const EROM_SIZE: usize = 0x1FE0;
pub const ERAM_SIZE: usize = 0x2000;
pub const PROM_SIZE: usize = 0x8000; // 32KB

pub const WRAM_BASE_ADDR: u16 = 0x0000;
pub const PPU_REG_BASE_ADDR: u16 = 0x2000;
pub const APU_IO_REG_BASE_ADDR: u16 = 0x4000;
pub const ROM_BASE_ADDR: u16 = 0x4020;


pub const PPU_CTRL_OFFSET: usize = 0x00;
pub const PPU_MASK_OFFSET: usize = 0x01;
pub const PPU_STATUS_OFFSET: usize = 0x02;
pub const PPU_OAMADDR_OFFSET: usize = 0x03;
pub const PPU_OAMDATA_OFFSET: usize = 0x04;
pub const PPU_SCROLL_OFFSET: usize = 0x05;
pub const PPU_ADDR_OFFSET: usize = 0x06;
pub const PPU_DATA_OFFSET: usize = 0x07;
pub const APU_IO_OAM_DMA_OFFSET: usize = 0x14;

use super::rom::*;
#[derive(Clone, Debug)]
pub struct System {
    //Memory for each component
    pub wram : [u8; 0x0800],
    pub ppu_reg: [u8; 0x0008],
    pub io_reg: [u8; 0x0018],
    pub rom : Rom,
    //Read/Write flags for each component
    
    pub write_oam_data: bool,
    pub write_ppu_scroll:bool,
    pub write_ppu_addr:bool,
    pub write_ppu_data:bool,
    pub write_oam_dma:bool,
    pub read_oam_data:bool,
    pub read_ppu_data:bool,

    //PPU registers
    pub ppu_is_second:bool,
    pub ppu_scroll_y:u8,
    pub ppu_addr_lower:u8,


}

impl System{
   pub fn default() -> Self {
        Self{
            wram:[0; 0x0800],
            ppu_reg:[0;0x0008],
            io_reg:[0;0x0018],
            rom: Rom::default(),
            write_oam_data: false,
            write_ppu_scroll:false,
            write_ppu_addr:false,
            write_ppu_data:false,
            write_oam_dma:false,
            read_oam_data:false,
            read_ppu_data:false,
            ppu_is_second:false,
            ppu_scroll_y:0,
            ppu_addr_lower:0,

        }

    }
}
impl System {

    pub fn reset(&mut self){
        self.wram = [0; WRAM_SIZE];
        self.ppu_reg = [0; PPU_REG_SIZE];
        self.io_reg = [0; APU_IO_REG_SIZE];

        self.write_oam_data = false;
        self.write_ppu_scroll = false;
        self.write_ppu_data = false;
        self.write_ppu_addr = false;
        self.write_oam_dma = false;
        self.read_oam_data = false;
        self.read_ppu_data = false;

        self.ppu_is_second = false;
        self.ppu_scroll_y = 0;
        self.ppu_addr_lower = 0;
    }
    pub fn write_ppu_vblank(&mut self, is_set : bool){
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x80u8;
        }else{
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x80u8);
            
        }
    }
    pub fn read_u8(&mut self, addr: u16, des : bool) -> u8{
        if addr < PPU_REG_BASE_ADDR { //This is the PPU register base address
            let index = usize::from(addr) % self.wram.len(); //Mirroring support
        //    log("reading opcode:");
         //  log(&self.wram[index].to_string());
            return self.wram[index];
        }else if addr < APU_IO_REG_BASE_ADDR {
            let index = usize::from(addr - PPU_REG_BASE_ADDR) % self.ppu_reg.len();
            match index {
                0x02 => {
                    let data = self.ppu_reg[index];
                    if !des {
                        self.ppu_is_second = false;
                        self.write_ppu_vblank(false);
                    }
                    data
                },
                0x04 => {
                    if !des {
                        self.read_oam_data = true
                    }
                    self.ppu_reg[index]
                },
                0x07 => {
                    if !des {
                        self.read_ppu_data = true
                    }
                    self.ppu_reg[index]

                },
                _=> self.ppu_reg[index],
            }
        } else if addr < ROM_BASE_ADDR {
            let index = usize::from(addr - APU_IO_REG_BASE_ADDR);
            if !des {
                match index {
                  //  0x16 => self.pad1.read_out(),
                  //  0x17 => self.pad2.read_out(),
                    _ => self.io_reg[index],
                }
            } else {
            self.io_reg[index]
            }
        }
        else {
            self.rom.read_u8(addr, des)
         }
    }

    pub fn write_u8(&mut self, addr: u16, data: u8, des : bool){
        if addr  > 0x6000 {
            log ("WROTE FROM 6000");
        }
        if addr < PPU_REG_BASE_ADDR {
            let index = usize::from(addr) % self.wram.len();
            self.wram[index] = data;
        } else if addr < APU_IO_REG_BASE_ADDR {
            let index = usize::from(addr - PPU_REG_BASE_ADDR) % self.ppu_reg.len();

            match index {
                0x04 => {
                    if !des {
                        self.write_oam_data = true
                    }
                    self.ppu_reg[index] = data;

                },
                0x05 => {
                    if self.ppu_is_second {
                        self.ppu_scroll_y = data;
                        if !des {
                            self.ppu_is_second  = false;
                            self.write_ppu_scroll = true;
                        }
                    }else {
                        self.ppu_reg[index] = data;
                        if !des {
                            self.ppu_is_second = true;
                        }
                    }
                },
                0x06 => {
                    if self.ppu_is_second {
                        self.ppu_addr_lower = data;
                        if !des {
                            self.ppu_is_second = false;
                            self.write_ppu_addr = true;
                        }
                    } else {
                        self.ppu_reg[index] = data;
                        if !des{
                            self.ppu_is_second = true
                        }
                    }
                },
                0x07 => {
                    self.ppu_reg[index] = data;
                    if !des {
                        self.write_ppu_data = true;
                    }
                },
                _ => {
                    self.ppu_reg[index] = data;
                }
            };
        } else if addr < ROM_BASE_ADDR {
            let index = usize::from(addr - APU_IO_REG_BASE_ADDR);
            if !des {
                match index {
                    0x14 => self.write_oam_data = true,
                   // 0x16 => self.pad1.write_strobe((data & 0x01) == 0x01),
                   // 0x16 => self.pad2.write_strobe((data & 0x01) == 0x01),
                    _ => {}
                }
            }
            self.io_reg[index] = data;
        } else {
            self.rom.write_u8(addr, data, des);
        }
    }
}