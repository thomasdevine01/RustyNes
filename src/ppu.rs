use super::cpu::*;
use super::system::*;
use super::video::*;

pub const CPU_CYCLE_PER_LINE: usize = (341 / 3); 

pub const NUM_OF_COLOR: usize = 3;

pub const VISIBLE_SCREEN_WIDTH: usize = 256;

pub const VISIBLE_SCREEN_HEIGHT: usize = 240;

pub const RENDER_SCREEN_WIDTH: u16 = VISIBLE_SCREEN_WIDTH as u16;

pub const RENDER_SCREEN_HEIGHT: u16 = 262; // 0 ~ 261

pub const PIXEL_PER_TILE: u16 = 8; // 1tile=8*8

pub const SCREEN_TILE_WIDTH: u16 = (VISIBLE_SCREEN_WIDTH as u16) / PIXEL_PER_TILE; // 256/8=32

pub const SCREEN_TILE_HEIGHT: u16 = (VISIBLE_SCREEN_HEIGHT as u16) / PIXEL_PER_TILE; // 240/8=30

pub const BG_NUM_OF_TILE_PER_ATTRIBUTE_TABLE_ENTRY: u16 = 4;

pub const ATTRIBUTE_TABLE_WIDTH: u16 =
    (SCREEN_TILE_WIDTH / BG_NUM_OF_TILE_PER_ATTRIBUTE_TABLE_ENTRY);


pub const OAM_SIZE: usize = 0x100;

/// 341cyc/513cyc*256byte=170.1byte
pub const OAM_DMA_COPY_SIZE_PER_PPU_STEP: u8 = 0xaa;

pub const PATTERN_TABLE_ENTRY_BYTE: u16 = 16;


pub const SPRITE_TEMP_SIZE: usize = 8;

pub const NUM_OF_SPRITE: usize = 64;

pub const SPRITE_SIZE: usize = 4;

pub const SPRITE_WIDTH: usize = 8;
pub const SPRITE_NORMAL_HEIGHT: usize = 8;
pub const SPRITE_LARGE_HEIGHT: usize = 16;

pub const CYCLE_PER_DRAW_FRAME: usize = CPU_CYCLE_PER_LINE * ((RENDER_SCREEN_HEIGHT + 1) as usize);