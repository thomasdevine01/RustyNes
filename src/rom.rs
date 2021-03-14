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
        let flags6  = read_f(6);
        let flags7  = read_f(7);
        let flags8  = read_f(8);
        let flags9  = read_f(9);
        let flags10 = read_f(10);

        true
    }
}