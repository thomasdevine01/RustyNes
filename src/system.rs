use super::rom::*;
#[derive(Clone)]

pub struct System {
    pub wram : [u8; 0x0800],
    pub ppu_reg: [u8; 0x0008],
    pub io_reg: [u8; 0x0018],
    pub rom : Rom,
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
}