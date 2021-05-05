use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::{io::Read, str};
use std::ptr;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;



pub mod system;
pub mod rom;
pub mod cpu;
pub mod instruction;
pub mod pad;
pub mod ppu;
pub mod video;
use crate::cpu::Cpu;
use crate::system::System;
use crate::pad::Pad;
use crate::ppu::*;
use crate::cpu::*;
use crate::pad::*;



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
        self.put_pixel(102, 100, 0xff00ff);
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

    pub fn status(&mut self, regn : u8)-> u16{
        match regn {
            0 => self.nes.cpu.regstat16(0),
            1 => self.nes.cpu.regstat16(1),
            2 => self.nes.cpu.regstat(0) as u16,
            3 => self.nes.cpu.regstat(1) as u16,
            4 => self.nes.cpu.regstat(2) as u16,
            5 => self.nes.cpu.regstat(3) as u16,
            6 => self.nes.system.rom.read_u8(0x6001, false) as u16,
            _ =>  222 as u16,
        }
    }
    pub fn reset(&mut self){
        log("Reset");
        self.nes.cpu.reset();
        self.nes.system.reset();
        self.nes.cpu.interrupt(&mut self.nes.system, cpu::Interrupt::RESET);
    }
    pub fn tick(&mut self){
        if self.running {
            
            self.nes.cpu.step(&mut self.nes.system);
            log(&self.nes.cpu.pc.to_string());    
        }
        self.render();

    }
    pub fn test(&self, key_code: u8){
        log("YOU PRESSED A BUTTON");
        log(&key_code.to_string());
    }
    pub fn load_rom(&mut self, data : &[u8]) -> bool{
        //self.rom.mem = data.to_vec();
        self.nes.system.rom.load_bin(|addr: usize| data[addr]);
        self.running = true;
        self.reset();
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

#[wasm_bindgen]
pub fn get_screen_width() -> usize {
    VISIBLE_SCREEN_WIDTH
}
#[wasm_bindgen]
pub fn get_screen_height() -> usize {
    VISIBLE_SCREEN_HEIGHT
}
#[wasm_bindgen]
pub fn get_num_of_colors() -> usize {
    NUM_OF_COLOR
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum KeyEvent {
    PressA,
    PressB,
    PressSelect,
    PressStart,
    PressUp,
    PressDown,
    PressLeft,
    PressRight,
    ReleaseA,
    ReleaseB,
    ReleaseSelect,
    ReleaseStart,
    ReleaseUp,
    ReleaseDown,
    ReleaseLeft,
    ReleaseRight,
}

#[wasm_bindgen]
pub struct wEmulator {
    fb: [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    cpu: Cpu,
    cpu_sys: System,
    //ppu: Ppu,
}

impl wEmulator {
    fn default() -> Self {
        Self {
            fb: [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
            cpu: Cpu::new(),
            cpu_sys: System::default(),
      //      ppu: Ppu::default(),
        }
    }
}
#[wasm_bindgen]
impl wEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> wEmulator {
        log("WasmEmulator::new()");
        wEmulator::default()
    }
    pub fn get_fb_ptr(&self) -> *const [[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH] {
        log("WasmEmulator::get_fb_ptr()");
        self.fb.as_ptr()
    }

    pub fn get_fb_size(&self) -> usize {
        log("WasmEmulator::get_fb_size()");
        NUM_OF_COLOR * VISIBLE_SCREEN_WIDTH * VISIBLE_SCREEN_HEIGHT
    }
 
    pub fn reset(&mut self) {
        log("WasmEmulator::reset()");
        self.fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];
        self.cpu.reset();
        self.cpu_sys.reset();
       // self.ppu.reset();
        self.cpu.interrupt(&mut self.cpu_sys, Interrupt::RESET);
    }

    pub fn load(&mut self, binary: &[u8]) -> bool {
        log("WasmEmulator::load()");
        let success = self
            .cpu_sys
            .rom
            .load_bin(|addr: usize| binary[addr]);
        if success {
            self.reset();
        }
        success
    }

    pub fn step_line(&mut self) {
        let mut total_cycle: usize = 0;
        while total_cycle < CYCLE_PER_DRAW_FRAME {
            // for debug
            // log("a:{:02X} x:{:02X} y:{:02X} pc:{:04X} sp:{:02X} p:{:02X} ", self.cpu.a, self.cpu.x, self.cpu.y, self.cpu.pc, self.cpu.sp, self.cpu.p);

            let cpu_cycle = usize::from(self.cpu.step(&mut self.cpu_sys));
           /* if let Some(interrupt) = self.ppu.step(cpu_cycle, &mut self.cpu_sys, &mut self.fb) {
                self.cpu.interrupt(&mut self.cpu_sys, interrupt);
            }*/
            total_cycle = total_cycle + cpu_cycle;
        }
    }
    pub fn update_key(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::PressA => self.cpu_sys.pad1.push_button(PadButton::A),
            KeyEvent::PressB => self.cpu_sys.pad1.push_button(PadButton::B),
            KeyEvent::PressSelect => self.cpu_sys.pad1.push_button(PadButton::Select),
            KeyEvent::PressStart => self.cpu_sys.pad1.push_button(PadButton::Start),
            KeyEvent::PressUp => self.cpu_sys.pad1.push_button(PadButton::Up),
            KeyEvent::PressDown => self.cpu_sys.pad1.push_button(PadButton::Down),
            KeyEvent::PressLeft => self.cpu_sys.pad1.push_button(PadButton::Left),
            KeyEvent::PressRight => self.cpu_sys.pad1.push_button(PadButton::Right),

            KeyEvent::ReleaseA => self.cpu_sys.pad1.release_button(PadButton::A),
            KeyEvent::ReleaseB => self.cpu_sys.pad1.release_button(PadButton::B),
            KeyEvent::ReleaseSelect => self.cpu_sys.pad1.release_button(PadButton::Select),
            KeyEvent::ReleaseStart => self.cpu_sys.pad1.release_button(PadButton::Start),
            KeyEvent::ReleaseUp => self.cpu_sys.pad1.release_button(PadButton::Up),
            KeyEvent::ReleaseDown => self.cpu_sys.pad1.release_button(PadButton::Down),
            KeyEvent::ReleaseLeft => self.cpu_sys.pad1.release_button(PadButton::Left),
            KeyEvent::ReleaseRight => self.cpu_sys.pad1.release_button(PadButton::Right),
        }
    }
}