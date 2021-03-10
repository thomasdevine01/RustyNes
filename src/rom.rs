#[derive(Debug, Clone)]
pub struct Rom{
    pub size : isize,
    pub mem : Vec<u8>,
}

impl Rom{
    pub fn new(size : isize, mem : Vec<u8>) -> Rom{
        Rom{
            size,
            mem,
        }
    }
}