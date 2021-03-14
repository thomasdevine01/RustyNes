/*
This is essentially the bus, and will become the core of the emulator, as it transmits data between each component
*/
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
    pub fn read_u8(&mut self, addr: u16, des : bool) -> u8{
        if addr < 0x2000 { //This is the PPU register base address
            let index = usize::from(addr) % self.wram.len(); //Mirroring support
            return self.wram[index];
        }else{
            return 0; //temporary,
        }
    }

    pub fn write_u8(&mut self, addr: u16, data: u8, des : bool){
        if addr < 0x2000 {
            let index = usize::from(addr) % self.wram.len();
            self.wram[index] = data;
        }else{
            
        }
    }
}