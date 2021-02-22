use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[derive(Debug, Clone)]
pub struct Cpu{
    pub memory: [u8; 4096]
}

impl Cpu{
    fn new(memory: &[u8; 4096]) -> Cpu{
        Cpu{
            memory: *memory,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Display{
    pub width : u16,
    pub height: u16,
    pub memory: [u32; 61440]
}

impl Display{
    fn new(width:u16, height:u16, memory: &[u32; 61440]) -> Display{
        Display{
            width,
            height,
            memory: *memory
        }
    }
}
static mut CPU: Cpu = Cpu {
    memory: [0; 4096]
};

pub struct Debug{
    pub val1 : usize,
    pub val2: u8
}

#[derive(Debug, Clone)]
struct Nes{
    cpu : Cpu,
    display : Display,

}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Emulator{
    nes: Nes,
    context: CanvasRenderingContext2d,
    pixel_width : u16,
    pixel_height : u16,

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


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
impl Emulator{
    fn build(
        canvas: &HtmlCanvasElement
    ) -> Emulator {
        let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        let display = Display::new(240, 255, &[0x000000; 61440]);
        let cpu = Cpu::new(&[0; 4096]);
        let nes = Nes{
            cpu : cpu,
            display: display,
            
        };
        let ret = Emulator{
            nes,
            context,
            pixel_width : 2,
            pixel_height: 2,
        };
        ret
    }
    fn render(&mut self){
        log("rendering");
        self.context.set_fill_style(&"0".into());
        self.context.fill_rect(0 as f64 ,0 as f64, self.nes.display.width as f64, self.nes.display.height as f64);
        self.put_pixel(100, 100, 0x00ff00);
    }
    fn put_pixel(&mut self, x:u32, y:u32, color:u32){
        self.context.save();
        let  s_colorstr = "#".to_string();
        let colorstr = s_colorstr +  &format!("{:0>6X}", color);
        self.context.set_fill_style(&colorstr.into());
        self.context.fill_rect(x as f64, y as f64, self.pixel_width as f64, self.pixel_height as f64);
        self.context.restore();
    }
}

#[wasm_bindgen]
impl Emulator{
    pub fn test(&mut self){
        log("test");
    }
    pub fn tick(&mut self){
        self.render();

    }
}
#[wasm_bindgen]
pub fn build_emulator() -> Emulator{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    Emulator::build(&canvas)
}