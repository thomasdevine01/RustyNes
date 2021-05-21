use super::rom::*;


pub const PATTERN_TABLE_BASE_ADDR: u16 = 0x0000;
pub const NAME_TABLE_BASE_ADDR: u16 = 0x2000;
pub const NAME_TABLE_MIRROR_BASE_ADDR: u16 = 0x3000;
pub const PALETTE_TABLE_BASE_ADDR: u16 = 0x3f00;
pub const VIDEO_ADDRESS_SIZE: u16 = 0x4000;

pub const NAME_TABLE_SIZE: usize = 0x0400;
pub const NUM_OF_NAME_TABLE: usize = 2;
pub const ATTRIBUTE_TABLE_SIZE: u16 = 0x0040;
pub const ATTRIBUTE_TABLE_OFFSET: u16 = 0x03c0; 

pub const PALETTE_SIZE: usize = 0x20;
pub const PALETTE_ENTRY_SIZE: u16 = 0x04;
pub const PALETTE_BG_OFFSET: u16 = 0x00;
pub const PALETTE_SPRITE_OFFSET: u16 = 0x10;
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
#[derive(Clone, Debug)]
pub struct VideoSystem {

    pub nametables: [[u8; NAME_TABLE_SIZE]; NUM_OF_NAME_TABLE],


    pub palette: [u8; PALETTE_SIZE],
}

impl Default for VideoSystem {
    fn default() -> Self {
        Self {
            nametables: [[0; NAME_TABLE_SIZE]; NUM_OF_NAME_TABLE],
            palette: [0; PALETTE_SIZE],
        }
    }
}

impl VideoSystem {
    pub fn reset(&mut self) {
        self.nametables = [[0; NAME_TABLE_SIZE]; NUM_OF_NAME_TABLE];
        self.palette = [0; PALETTE_SIZE];
    }
}

impl VideoSystem {

    fn convert_name_table_addr(&self, mirror_mode: MirrorTable, addr: u16) -> (usize, usize) {
        debug_assert!(addr >= NAME_TABLE_BASE_ADDR);
        debug_assert!(addr < NAME_TABLE_MIRROR_BASE_ADDR);

        let offset = usize::from(addr - NAME_TABLE_BASE_ADDR) % NAME_TABLE_SIZE;
        let table_index = match mirror_mode {
            MirrorTable::Horizontal => {
                // [A, A]
                // [B, B]
                if addr < 0x2800 {
                    0
                } else {
                    1
                }
            }
            MirrorTable::Vertical => {
                // [A, B]
                // [A, B]
                let tmp_addr = if addr >= 0x2800 { addr - 0x800 } else { addr }; 
                if tmp_addr < 0x2400 {
                    0
                } else {
                    1
                }
            }
            MirrorTable::SingleScreen => {
                // [A, A]
                // [A, A]
                0
            }
            MirrorTable::FourScreen => {
                // [A, B]
                // [C, D]
                usize::from((addr - 0x2000) / 4)
            }
            _ => {
                unimplemented!();
            }
        };
        (table_index, offset)
    }
    pub fn read_u8(&self, rom: &mut Rom, addr: u16) -> u8 {
        debug_assert!(addr < VIDEO_ADDRESS_SIZE);

        if addr < NAME_TABLE_BASE_ADDR {
            rom.read_video_u8(addr)
        } else if addr < NAME_TABLE_MIRROR_BASE_ADDR {
            let (index, offset) = self.convert_name_table_addr(rom.mirror_table, addr);
            self.nametables[index][offset]
        } else if addr < PALETTE_TABLE_BASE_ADDR {
            let (index, offset) =
                self.convert_name_table_addr(rom.mirror_table, addr - 0x1000);
            self.nametables[index][offset]
        } else {
            let index = usize::from(addr - PALETTE_TABLE_BASE_ADDR) % PALETTE_SIZE;
            match index {
                0x10 => self.palette[0x00],
                0x14 => self.palette[0x04],
                0x18 => self.palette[0x08],
                0x1c => self.palette[0x0c],
                _ => arr_read!(self.palette, index),
            }
        }
    }
    pub fn write_u8(&mut self, rom: &mut Rom, addr: u16, data: u8) {
        debug_assert!(addr < VIDEO_ADDRESS_SIZE);

        if addr < NAME_TABLE_BASE_ADDR {
            rom.write_video_u8(addr, data);
        } else if addr < NAME_TABLE_MIRROR_BASE_ADDR {
            let (index, offset) = self.convert_name_table_addr(rom.mirror_table, addr);
            self.nametables[index][offset] = data;
        } else if addr < PALETTE_TABLE_BASE_ADDR {
           
            let (index, offset) =
                self.convert_name_table_addr(rom.mirror_table, addr - 0x1000);
            self.nametables[index][offset] = data;
        } else {
            let index = usize::from(addr - PALETTE_TABLE_BASE_ADDR) % PALETTE_SIZE;
          
            match index {
                0x10 => self.palette[0x00] = data,
                0x14 => self.palette[0x04] = data,
                0x18 => self.palette[0x08] = data,
                0x1c => self.palette[0x0c] = data,
                _ => arr_write!(self.palette, index, data),
            };
        }
    }
}