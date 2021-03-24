use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
#[derive(Debug, Clone)]
pub enum Mapper{
    Unknown,
    Nrom,
}
#[derive(Clone, Debug)]
pub enum MirrorTable{
    Unknown,
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen,
}

#[derive(Clone, Debug)]
pub struct Rom{
    pub mapper: Mapper,
    pub mirror_table: MirrorTable,
    pub bat_pack : bool,
    pub p_rom_bytes : usize,
    pub c_rom_bytes : usize,
    pub p_rom:[u8; 0x8000],
    pub c_rom:[u8; 0x2000],
    pub b_pack:[u8;0x2000],
}

impl Rom{
    pub fn default() -> Self {
        Self{
            mapper : Mapper::Unknown,
            mirror_table: MirrorTable::Unknown,
            bat_pack : false,
            p_rom_bytes: 0,
            c_rom_bytes : 0,
            p_rom: [0; 0x8000],
            c_rom: [0; 0x2000],
            b_pack: [0; 0x2000],
        }
    }
    pub fn load_bin(&mut self, read_f:impl Fn(usize) -> u8) -> bool{

        //Get the N E S bits and the line break
        if(read_f(0) != 0x4e) {
            return false;
        }
        if(read_f(1) != 0x45) {
            return false;
        }
        if(read_f(2) != 0x53) {
            return false;
        }
        if(read_f(3) != 0x1a) {
            return false;
        }
        let p_rom_sz = usize::from(read_f(4));
        let c_rom_sz = usize::from(read_f(5));
        let flags6  = read_f(6); // Mapper 1
        let flags7  = read_f(7); // Mapper 2
        let flags8  = read_f(8); // Ram Size
        let flags9  = read_f(9); // tv system 1
        let flags10 = read_f(10); // tv system 2
        
        let is_vert_m = (flags6 & 0x01) == 0x01;
        if (is_vert_m){
            self.mirror_table = MirrorTable::Vertical;
        }else{
            self.mirror_table = MirrorTable::Horizontal;
        }
        self.bat_pack = (flags6 & 0x02) == 0x02;
        let trainer_exists = (flags6 & 0x04) == 0x04;
        let header_bytes = 16;
        let trainer_bytes = if trainer_exists {512} else {0};
        let trainer_base_addr = header_bytes;

        let p_rom_bytes = p_rom_sz * 0x4000;
        let chr_rom_bytes = c_rom_sz * 0x2000;
        let p_rom_base = header_bytes + trainer_bytes;
        let c_rom_base = header_bytes + trainer_bytes + p_rom_bytes;

        self.mapper = Mapper::Nrom;
        let mut bytes_read = 0;
        if trainer_exists {
            for i in 0..INES_TRAINER_DATA_SIZE {
                let ines_binary_addr = trainer_base_addr + i;
                self.p_rom[i] = read_f(ines_binary_addr);
            }
        }
        //Program rom
        for i in 0..p_rom_bytes {
            let bin_addr = p_rom_base + i;
            self.p_rom[i] = read_f(bin_addr);
            bytes_read = i;
        }
        
        log("read in ");
        log(&bytes_read.to_string());
        log("bytes for p_rom");
        //Character rom
        let mut bytes_read_p = 0;
        for i in 0..chr_rom_bytes{
            let bin_addr = c_rom_base + i;
            self.c_rom[i] = read_f(bin_addr);
            bytes_read_p = i;
        }
        log("read in ");
        log(&bytes_read_p.to_string());
        log("bytes for c_rom");
        self.p_rom_bytes= p_rom_bytes;
        self.c_rom_bytes = chr_rom_bytes;

        true
    }

    pub fn read_u8(&mut self, addr: u16, _des : bool) -> u8 {
        if addr < PRG_ROM_SYSTEM_BASE_ADDR {
            let index = usize::from(addr - BATTERY_PACKED_RAM_BASE_ADDR);
            self.b_pack[index]
        } else {
            let index = usize::from(addr - PRG_ROM_SYSTEM_BASE_ADDR);
            if index < self.p_rom_bytes {
                self.p_rom[index]
            } else {
                self.p_rom[index - self.p_rom_bytes]
            }
        }
    }
    pub fn write_u8(&mut self, addr: u16, data: u8, _des : bool)  {
        if addr < PRG_ROM_SYSTEM_BASE_ADDR {
            let index = usize::from(addr - BATTERY_PACKED_RAM_BASE_ADDR);
            self.b_pack[index] = data
        }else {
            let index = usize::from(addr - PRG_ROM_SYSTEM_BASE_ADDR);

            if index < self.p_rom_bytes {
                self.p_rom[index] = data;
            }else{
                self.p_rom[index - self.p_rom_bytes] = data;
            }
        }
    }

}