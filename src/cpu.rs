#[derive(Debug, Clone)]
pub struct Cpu{
    pub memory: [u8; 4096],
    pub PC : u16, //2-byte program counter
    X : u8,
    Y : u8, //X and Y are index registers
    A : u8, //Accumulator
    S : u8, //Stack Pointer
    P : u8, //Status Register
}


impl Cpu{
    pub fn new(memory: &[u8; 4096]) -> Cpu{
        Cpu{
            memory: *memory,
            PC : 0,
            X : 0,
            Y : 0,
            A : 0,
            S : 0,
            P : 0,
        }
    }
}