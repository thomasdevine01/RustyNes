use wasm_bindgen::prelude::*;


pub const PRG_ROM_MAX_SIZE: usize = 0x8000;
pub const CHR_ROM_MAX_SIZE: usize = 0x2000;
pub const BATTERY_PACKED_RAM_MAX_SIZE: usize = 0x2000;

pub const PRG_ROM_SYSTEM_BASE_ADDR: u16 = 0x8000;
pub const BATTERY_PACKED_RAM_BASE_ADDR: u16 = 0x6000;

pub const INES_TRAINER_DATA_SIZE: usize = 0x0200;
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
#[derive(Debug, Clone)]
pub enum Mapper{
    Unknown,
    Nrom,
}
//Defines the nametable mirroring pattern.
//http://wiki.nesdev.com/w/index.php/Mirroring#Nametable_Mirroring
#[derive(Copy, Clone, Debug)]
pub enum MirrorTable{
    Unknown,
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen,
}

//This is the "game cartdridge" structure, in INES format
//http://wiki.nesdev.com/w/index.php/INES
#[derive(Clone, Debug)]
pub struct Rom{
    //Which mapper we have defined, this emulator only supports mapper 0 (the most basic of games like original Super Mario)
    pub mapper: Mapper,
    //The mirror table, see the mirror table enum above.
    pub mirror_table: MirrorTable,
    //The SRAM
    pub sram : bool,
    //Program  memory size
    pub p_rom_bytes : usize,
    //Character memory size, these are the graphics
    pub c_rom_bytes : usize,
    //Actual program on the rom
    pub p_rom:[u8; PRG_ROM_MAX_SIZE],
    //Actual graphics
    pub c_rom:[u8; CHR_ROM_MAX_SIZE],
    //The ram we can modify on the ROM (I know, I know)
    pub srambytes:[u8;BATTERY_PACKED_RAM_MAX_SIZE],
}

impl Rom{
    pub fn default() -> Self {
        Self{
            mapper : Mapper::Unknown,
            mirror_table: MirrorTable::Unknown,
            sram : false,
            p_rom_bytes: 0,
            c_rom_bytes : 0,
            p_rom: [0; PRG_ROM_MAX_SIZE],
            c_rom: [0; CHR_ROM_MAX_SIZE],
            srambytes: [0; BATTERY_PACKED_RAM_MAX_SIZE],
        }
    }
    pub fn load_bin(&mut self, read_f:impl Fn(usize) -> u8) -> bool{

        //Get the N E S bits and the line break
        if read_f(0) != 0x4e {
            return false;
        }
        if read_f(1) != 0x45 {
            return false;
        }
        if read_f(2) != 0x53 {
            return false;
        }
        if read_f(3) != 0x1a {
            return false;
        }
        //4th and 5th bytes are the sizes of prg and chr rom in 16k increments
        //I.E if p_rom_sz is 2 then there are 32k bytes in PRG rom
        let p_rom_sz = usize::from(read_f(4));
        //Same here except its 8k increments
        let c_rom_sz = usize::from(read_f(5));
        //Mapper flag 1, we only support 1 mapper and rom type so this is the only flag we actually use
        let flags6  = read_f(6); // Mapper 1
        let _flags7  = read_f(7); // Mapper 2
        let _flags8  = read_f(8); // Ram Size
        let _flags9  = read_f(9); // tv system 1
        let _flags10 = read_f(10); // tv system 2
        debug_assert!(p_rom_sz > 0);
        //Are we mirroring vertically?
        let is_vert_m = (flags6 & 0x01) == 0x01;
        if is_vert_m{
            self.mirror_table = MirrorTable::Vertical;
        }else{
            self.mirror_table = MirrorTable::Horizontal;
        }
        self.sram = (flags6 & 0x02) == 0x02;
        let trainer_exists = (flags6 & 0x04) == 0x04;
        let header_bytes = 16;
        //I am not sure what a trainer is in this context. But its in the INES spec so we have to account for the bytes
        let trainer_bytes = if trainer_exists { 512 } else { 0 };
        let prg_rom_bytes = p_rom_sz * 0x4000; 
        let chr_rom_bytes = c_rom_sz * 0x2000; 
        let trainer_baseaddr = header_bytes;
        let prg_rom_baseaddr = header_bytes + trainer_bytes;
        let chr_rom_baseaddr = header_bytes + trainer_bytes + prg_rom_bytes;

        self.mapper = Mapper::Nrom;
        debug_assert!(prg_rom_bytes <= PRG_ROM_MAX_SIZE);
        debug_assert!(chr_rom_bytes <= CHR_ROM_MAX_SIZE);
        //Load everything in
        if trainer_exists {
            for i in 0..INES_TRAINER_DATA_SIZE {
                let ines_binary_addr = trainer_baseaddr + i;
                self.p_rom[i] = read_f(ines_binary_addr);
            }
        }
        for i in 0..prg_rom_bytes {
            let bin_addr = prg_rom_baseaddr + i;
            self.p_rom[i] = read_f(bin_addr);
           
        }
            
        for i in 0..chr_rom_bytes{
            let bin_addr = chr_rom_baseaddr + i;
            self.c_rom[i] = read_f(bin_addr);
            
        }

        self.p_rom_bytes= prg_rom_bytes;
        self.c_rom_bytes = chr_rom_bytes;

        true
    }
    //Read 8 bytes from ROM, mapped out appropriately
   pub fn read_u8(&mut self, addr: u16, _is_nondestructive: bool) -> u8 {
        if addr < PRG_ROM_SYSTEM_BASE_ADDR {
            debug_assert!(addr >= BATTERY_PACKED_RAM_BASE_ADDR);

            let index = usize::from(addr - BATTERY_PACKED_RAM_BASE_ADDR);
            arr_read!(self.srambytes, index)
        } else {
            debug_assert!(addr >= PRG_ROM_SYSTEM_BASE_ADDR);

            let index = usize::from(addr - PRG_ROM_SYSTEM_BASE_ADDR);
        
            if index < self.p_rom_bytes {
                arr_read!(self.p_rom, index)
            } else {
                arr_read!(self.p_rom, index - self.p_rom_bytes)
            }
        }
    }
    //Same as above for write
    pub fn write_u8(&mut self, addr: u16, data: u8, _is_nondestructive: bool) {
        if addr < PRG_ROM_SYSTEM_BASE_ADDR {
            debug_assert!(addr >= BATTERY_PACKED_RAM_BASE_ADDR);

            let index = usize::from(addr - BATTERY_PACKED_RAM_BASE_ADDR);
            arr_write!(self.srambytes, index, data)
        } else {
            debug_assert!(addr >= PRG_ROM_SYSTEM_BASE_ADDR);

            let index = usize::from(addr - PRG_ROM_SYSTEM_BASE_ADDR);

            if index < self.p_rom_bytes {
                arr_write!(self.p_rom, index, data);
            } else {
                arr_write!(self.p_rom, index - self.p_rom_bytes, data);
            }
        }
    }
    //Reads and writes to graphics memory
    pub fn read_video_u8(&mut self, addr: u16) -> u8 {
            let index = usize::from(addr);
            debug_assert!(index < CHR_ROM_MAX_SIZE);
            arr_read!(self.c_rom, index)
        }
       
    pub fn write_video_u8(&mut self, addr: u16, data: u8) {
            let index = usize::from(addr);
            debug_assert!(index < CHR_ROM_MAX_SIZE);
            arr_write!(self.c_rom, index, data);
        }
      
    pub fn reset(&mut self) {
            self.mapper = Mapper::Unknown;
            self.mirror_table = MirrorTable::Unknown;
            self.sram = false;
            self.p_rom_bytes = 0;
            self.c_rom_bytes = 0;
            self.p_rom = [0; PRG_ROM_MAX_SIZE];
            self.c_rom = [0; CHR_ROM_MAX_SIZE];
            self.srambytes = [0; BATTERY_PACKED_RAM_MAX_SIZE];
        }

}