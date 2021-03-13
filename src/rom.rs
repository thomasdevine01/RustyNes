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
}