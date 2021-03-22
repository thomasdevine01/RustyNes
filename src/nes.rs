use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::{io::Read, str};
use std::ptr;


pub mod system;
pub mod rom;
pub mod cpu;
pub mod instruction;
use crate::cpu::Cpu;
use crate::system::System;

#[derive(Debug, Clone)]
pub struct Display{
    pub width : u16,
    pub height: u16,
    pub memory: [u32; 61440]
}
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
impl Display{
    fn new(width:u16, height:u16, memory: &[u32; 61440]) -> Display{
        Display{
            width,
            height,
            memory: *memory
        }
    }
}
pub struct Debug{
    pub val1 : usize,
    pub val2: u8
}

#[derive(Debug, Clone)]
struct Nes{
    cpu : Cpu,
    system : System,
    display : Display,

}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Emulator{
    nes: Nes,
    context: CanvasRenderingContext2d,
    pixel_width : u16,
    pixel_height : u16,
    running : bool,
    
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
        let cpu = Cpu::new();
        let sys = System::default();
        let nes = Nes{
            cpu : cpu,
            display: display,
            system : sys,
            
        };
        let ret = Emulator{
            nes,
            context,
            pixel_width : 2,
            pixel_height: 2,
            running : false,
        };
        ret
    }
    fn render(&mut self){
        
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

    pub fn status(&self, regn : u8)-> u16{
        match regn {
            0 => self.nes.cpu.regstat16(0),
            1 => self.nes.cpu.regstat16(1),
            2 => self.nes.cpu.regstat(0) as u16,
            3 => self.nes.cpu.regstat(1) as u16,
            4 => self.nes.cpu.regstat(2) as u16,
            5 => self.nes.cpu.regstat(3) as u16,
            _ =>  222 as u16,
        }
    }
    pub fn tick(&mut self){
        if self.running {
            
            self.nes.cpu.step(&mut self.nes.system);
            log(&self.nes.cpu.pc.to_string());    
        }
        self.render();

    }
    pub fn load_rom(&mut self, data : &[u8]) -> bool{
        //self.rom.mem = data.to_vec();
        self.nes.system.rom.load_bin(|addr: usize| data[addr]);
        self.running = true;
        true
        //log(&self.rom.mem.len().to_string());
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


