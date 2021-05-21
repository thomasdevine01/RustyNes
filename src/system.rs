
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
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

use crate::video::VideoSystem;

use super::rom::*;
use super::pad::*;
#[derive(Clone, Debug)]
pub struct System {
    //Memory for each component
    pub wram : [u8; WRAM_SIZE],
    pub ppu_reg: [u8; PPU_REG_SIZE],
    pub io_reg: [u8; APU_IO_REG_SIZE],
    pub rom : Rom,
    pub video: VideoSystem,
    //Pads
    pub pad1: Pad,
    pub pad2: Pad,
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
            wram:[0; WRAM_SIZE],
            ppu_reg:[0;PPU_REG_SIZE],
            io_reg:[0;APU_IO_REG_SIZE],
            rom: Rom::default(),
            pad1: Pad::default(),
            pad2: Pad::default(),
            video: VideoSystem::default(),
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
        self.video.reset();
        self.pad1.reset();
        self.pad2.reset();
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

    pub fn read_u8(&mut self, addr: u16, is_nondestructive: bool) -> u8 {
        if addr < PPU_REG_BASE_ADDR {
     
            let index = usize::from(addr) % self.wram.len();
            arr_read!(self.wram, index)
        } else if addr < APU_IO_REG_BASE_ADDR {
      
            let index = usize::from(addr - PPU_REG_BASE_ADDR) % self.ppu_reg.len();
            debug_assert!(index < 0x9);
            match index {
             
                0x02 => {
                    let data = self.ppu_reg[index]; 
                    if !is_nondestructive {
                        self.ppu_is_second = false;
                        self.write_ppu_is_vblank(false);
                    }
                    data
                }
    
                0x04 => {
                    if !is_nondestructive {
                        self.read_oam_data = true;
                    }
                    arr_read!(self.ppu_reg, index)
                }
  
                0x07 => {
                    if !is_nondestructive {
                        self.read_ppu_data = true;
                    }
                    arr_read!(self.ppu_reg, index)
                }
        
                _ => arr_read!(self.ppu_reg, index),
            }
        } else if addr < ROM_BASE_ADDR {
            let index = usize::from(addr - APU_IO_REG_BASE_ADDR);
            if !is_nondestructive {
                match index {
                    
                    0x16 => self.pad1.read_out(), // pad1
                    0x17 => self.pad2.read_out(), // pad2
                    _ => arr_read!(self.io_reg, index),
                }
            } else {
                arr_read!(self.io_reg, index)
            }
        } else {
            self.rom.read_u8(addr, is_nondestructive)
        }
    }

    pub fn write_u8(&mut self, addr: u16, data: u8, is_nondestructive: bool) {
        if addr < PPU_REG_BASE_ADDR {
            // mirror support
            let index = usize::from(addr) % self.wram.len();
            arr_write!(self.wram, index, data);
        } else if addr < APU_IO_REG_BASE_ADDR {
            // mirror support
            let index = usize::from(addr - PPU_REG_BASE_ADDR) % self.ppu_reg.len();
            match index {
            
                0x04 => {
                    if !is_nondestructive {
                        self.write_oam_data = true
                    }
                    arr_write!(self.ppu_reg, index, data);
                }
              
                0x05 => {
                    if self.ppu_is_second {
                        self.ppu_scroll_y = data;
                        if !is_nondestructive {
                            self.ppu_is_second = false;
                     
                            self.write_ppu_scroll = true;
                        }
                    } else {
                        arr_write!(self.ppu_reg, index, data);
                        if !is_nondestructive {
                            self.ppu_is_second = true;
                        }
                    }
                }
      
                0x06 => {
                    if self.ppu_is_second {
                        self.ppu_addr_lower = data;
                        if !is_nondestructive {
                            self.ppu_is_second = false;
                        
                            self.write_ppu_addr = true;
                        }
                    } else {
                        arr_write!(self.ppu_reg, index, data);
                        if !is_nondestructive {
                            self.ppu_is_second = true;
                        }
                    }
                }
     
                0x07 => {
                    arr_write!(self.ppu_reg, index, data);
                    if !is_nondestructive {
            
                        self.write_ppu_data = true;
                    }
                }
        
                _ => {
                    arr_write!(self.ppu_reg, index, data);
                }
            };
        } else if addr < ROM_BASE_ADDR {
            let index = usize::from(addr - APU_IO_REG_BASE_ADDR);
            if !is_nondestructive {
                match index {
                
                    0x14 => self.write_oam_dma = true, 
                    0x16 => self.pad1.write_strobe((data & 0x01) == 0x01), 
                    0x17 => self.pad2.write_strobe((data & 0x01) == 0x01), 
                    _ => {}
                }
            }
            arr_write!(self.io_reg, index, data);
        } else {
            self.rom.write_u8(addr, data, is_nondestructive);
        }
    }
}

//PPU registers
impl System {

    pub fn read_ppu_nmi_enable(&self) -> bool {
        (self.ppu_reg[PPU_CTRL_OFFSET] & 0x80u8) == 0x80u8
    }

    pub fn read_ppu_is_master(&self) -> bool {
        (self.ppu_reg[PPU_CTRL_OFFSET] & 0x40u8) == 0x40u8
    }

    pub fn read_ppu_sprite_height(&self) -> u8 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x20u8) == 0x20u8 {
            16
        } else {
            8
        }
    }
    pub fn read_ppu_bg_pattern_table_addr(&self) -> u16 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x10u8) == 0x10u8 {
            0x1000u16
        } else {
            0x0000u16
        }
    }
    pub fn read_ppu_sprite_pattern_table_addr(&self) -> u16 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x08u8) == 0x08u8 {
            0x1000u16
        } else {
            0x0000u16
        }
    }

    pub fn read_ppu_addr_increment(&self) -> u8 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x04u8) == 0x04u8 {
            32u8
        } else {
            1u8
        }
    }
    pub fn read_ppu_name_table_base_addr(&self) -> u16 {
        match self.ppu_reg[PPU_CTRL_OFFSET] & 0x03u8 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("invalid name table addr index"),
        }
    }

    pub fn read_ppu_is_write_sprite(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x10u8) == 0x10u8
    }

    pub fn read_ppu_is_write_bg(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x08u8) == 0x08u8
    }

    pub fn read_ppu_is_clip_sprite_leftend(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x04u8) != 0x04u8
    }

    pub fn read_ppu_is_clip_bg_leftend(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x02u8) != 0x02u8
    }
    pub fn read_is_monochrome(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x01u8) == 0x01u8
    }

    pub fn read_ppu_is_vblank(&self) -> bool {
        (self.ppu_reg[PPU_STATUS_OFFSET] & 0x80u8) == 0x80u8
    }

    pub fn write_ppu_is_vblank(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x80u8;
        } else {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x80u8);
        }
    }

    pub fn read_ppu_is_hit_sprite0(&self) -> bool {
        (self.ppu_reg[PPU_STATUS_OFFSET] & 0x40u8) == 0x40u8
    }
    pub fn write_ppu_is_hit_sprite0(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x40u8;
        } else {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x40u8);
        }
    }

    pub fn read_ppu_is_sprite_overflow(&self) -> bool {
        (self.ppu_reg[PPU_STATUS_OFFSET] & 0x20u8) == 0x20u8
    }
    pub fn write_ppu_is_sprite_overflow(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x20u8;
        } else {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x20u8);
        }
    }

    pub fn clear_ppu_status(&mut self) {
        self.ppu_reg[PPU_STATUS_OFFSET] = 0x00u8;
    }

    pub fn read_ppu_oam_addr(&self) -> u8 {
        self.ppu_reg[PPU_OAMADDR_OFFSET]
    }

    pub fn read_oam_data(&mut self) -> (bool, bool, u8) {

        if self.write_oam_data {
            self.write_oam_data = false;
            (false, true, self.ppu_reg[PPU_OAMDATA_OFFSET])
        } else if self.read_oam_data {
            self.read_oam_data = false;
            (true, false, self.ppu_reg[PPU_OAMDATA_OFFSET])
        } else {
            (false, false, self.ppu_reg[PPU_OAMDATA_OFFSET])
        }
    }

    pub fn write_oam_data(&mut self, data: u8) {
        self.ppu_reg[PPU_OAMDATA_OFFSET] = data;
    }


    pub fn read_ppu_scroll(&mut self) -> (bool, u8, u8) {
        if self.write_ppu_scroll {
            self.write_ppu_scroll = false;
            (true, self.ppu_reg[PPU_SCROLL_OFFSET], self.ppu_scroll_y)
        } else {
            (
                false,
                self.ppu_reg[PPU_SCROLL_OFFSET],
                self.ppu_scroll_y,
            )
        }
    }

    pub fn read_ppu_addr(&mut self) -> (bool, u16) {
        let addr =
            (u16::from(self.ppu_reg[PPU_ADDR_OFFSET]) << 8) | u16::from(self.ppu_addr_lower);
        if self.write_ppu_addr {
            self.write_ppu_addr = false;
            (true, addr)
        } else {
            (false, addr)
        }
    }

    pub fn read_ppu_data(&mut self) -> (bool, bool, u8) {

        if self.write_ppu_data {
            self.write_ppu_data = false;
            (false, true, self.ppu_reg[PPU_DATA_OFFSET])
        } else if self.read_ppu_data {
            self.read_ppu_data = false;
            (true, false, self.ppu_reg[PPU_DATA_OFFSET])
        } else {
            (false, false, self.ppu_reg[PPU_DATA_OFFSET])
        }
    }

  
    pub fn write_ppu_data(&mut self, data: u8) {
        self.ppu_reg[PPU_DATA_OFFSET] = data;
    }

 
    pub fn increment_ppu_addr(&mut self) {
        let current_addr =
            (u16::from(self.ppu_reg[PPU_ADDR_OFFSET]) << 8) | u16::from(self.ppu_addr_lower);
      
        let add_val = u16::from(self.read_ppu_addr_increment());
        let dst_addr = current_addr.wrapping_add(add_val);
   
        self.ppu_addr_lower = (dst_addr & 0xff) as u8;
        self.ppu_reg[PPU_ADDR_OFFSET] = (dst_addr >> 8) as u8;
    }
 
    pub fn read_oam_dma(&mut self) -> (bool, u16) {
        let start_addr = u16::from(self.io_reg[APU_IO_OAM_DMA_OFFSET]) << 8;
        if self.write_oam_dma {
            self.write_oam_dma = false;
            (true, start_addr)
        } else {
            (false, start_addr)
        }
    }
}
