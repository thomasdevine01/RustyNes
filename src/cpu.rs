#[derive(Debug, Clone)]
pub struct Cpu{
    pub memory: [u8; 4096]
}


impl Cpu{
    pub fn new(memory: &[u8; 4096]) -> Cpu{
        Cpu{
            memory: *memory,
        }
    }
}