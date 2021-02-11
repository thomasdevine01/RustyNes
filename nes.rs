pub struct Cpu{
    pub memory: [u8; 4096]
}
pub struct Display{
    pub width : u16,
    pub height: u16,
    pub memory: [u32; 61440]
}

static mut CPU: Cpu = Cpu {
    memory: [0; 4096]
};

pub struct Debug{
    pub val1 : usize,
    pub val2: u8
}

static mut DEBUG: Debug = Debug{
    val1: 0,
    val2 : 0,
};
static mut DISPLAY : Display = Display{
    width:240,
    height:255,
    memory: [0x000000; 61440]
};

#[no_mangle]
pub fn get_memory() -> &'static [u8; 4096]{
    unsafe{
        &CPU.memory
    }
}

#[no_mangle]
pub fn get_debug() ->usize {
    unsafe{
        DEBUG.val1
    }
}
#[no_mangle]
pub fn get_display() -> &'static [u32;61440]{
    unsafe{
        &DISPLAY.memory
    }
}
#[no_mangle]
pub fn modify_memory(x:usize, y:u8){
    unsafe{
    CPU.memory[x] = y;
    }
}
#[no_mangle]
pub fn put_pixel(x:u16, y:u16, color:u32){
    unsafe{
        let pos:usize  = (DISPLAY.width * x + y).into();
        DEBUG.val1 = pos;
        DISPLAY.memory[pos] = color;
    }
}

fn main(){
    
}