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

#[derive(Copy, Clone)]
pub struct Position(pub u8, pub u8);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);
impl Color {

    pub fn from(src: u8) -> Color {
        let index = src & 0x3f;
        let table: [Color; 0x40] = include!("ppu_palette.rs");
        table[index as usize]
    }
    pub fn is_black(&self) -> bool {
        self.0 == 0x0 && self.1 == 0x0 && self.2 == 0x0
    }
}


#[derive(Copy, Clone)]
pub enum TileId {

    Normal { id: u8 },

    Large {

        pattern_table_addr: u16,

        upper_tile_id: u8,

        lower_tile_id: u8,
    },
}
impl TileId {
    pub fn normal(src: u8) -> TileId {
        TileId::Normal { id: src }
    }
    pub fn large(src: u8) -> TileId {
        TileId::Large {
            pattern_table_addr: (if (src & 0x01) == 0x01 {
                0x1000
            } else {
                0x0000u16
            }),
            upper_tile_id: src & 0xfe,
            lower_tile_id: (src & 0xfe) + 1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SpriteAttr {

    is_vert_flip: bool,

    is_hor_flip: bool,

    is_draw_front: bool,

    palette_id: u8,
}
impl SpriteAttr {
    pub fn from(src: u8) -> SpriteAttr {
        SpriteAttr {
            is_vert_flip: (src & 0x80) == 0x80,
            is_hor_flip: (src & 0x40) == 0x40,
            is_draw_front: (src & 0x20) != 0x20,
            palette_id: (src & 0x03),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sprite {

    y: u8,

    tile_id: TileId,

    attr: SpriteAttr,

    x: u8,
}

impl Sprite {

    pub fn from(is_large: bool, byte0: u8, byte1: u8, byte2: u8, byte3: u8) -> Sprite {
        Sprite {
            y: byte0,
            tile_id: (if is_large {
                TileId::large(byte1)
            } else {
                TileId::normal(byte1)
            }),
            attr: SpriteAttr::from(byte2),
            x: byte3,
        }
    }
}

#[derive(Copy, Clone)]
enum LineStatus {
    Visible,                // 0~239
    PostRender,             // 240
    VerticalBlanking(bool), // 241~260
    PreRender,              // 261
}

impl LineStatus {
    fn from(line: u16) -> LineStatus {
        if line < 240 {
            LineStatus::Visible
        } else if line == 240 {
            LineStatus::PostRender
        } else if line < 261 {
            LineStatus::VerticalBlanking(line == 241)
        } else if line == 261 {
            LineStatus::PreRender
        } else {
            panic!("invalid line status");
        }
    }
}