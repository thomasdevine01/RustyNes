
/* BEWARE ALL YE WHO ENTER HERE, YOU'LL NEED THIS */
/* https://wiki.nesdev.com/w/index.php/PPU_programmer_reference */

use super::cpu::*;
use super::system::*;
use super::video::*;

pub const CPU_CYCLE_PER_LINE: usize = 341 / 3; 

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
    SCREEN_TILE_WIDTH / BG_NUM_OF_TILE_PER_ATTRIBUTE_TABLE_ENTRY;


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

#[derive(Clone)]
pub struct Ppu {
    
    pub oam: [u8; OAM_SIZE],
   
    pub sprite_temps: [Option<Sprite>; SPRITE_TEMP_SIZE],

   
    pub cumulative_cpu_cyc: usize,
    
    pub current_line: u16,

   
    pub fetch_scroll_x: u8,
    pub fetch_scroll_y: u8,
    pub current_scroll_x: u8,
    pub current_scroll_y: u8,


    pub is_dma_running: bool,
   
    pub dma_cpu_src_addr: u16,

    pub dma_oam_dst_addr: u8,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; OAM_SIZE],
            sprite_temps: [None; SPRITE_TEMP_SIZE],

            cumulative_cpu_cyc: 0,
            current_line: 241,

            fetch_scroll_x: 0,
            fetch_scroll_y: 0,
            current_scroll_x: 0,
            current_scroll_y: 0,

            is_dma_running: false,
            dma_cpu_src_addr: 0,
            dma_oam_dst_addr: 0,
        }
    }
}

impl Ppu {
   pub fn reset(&mut self) {
        self.oam = [0; OAM_SIZE];
        self.sprite_temps = [None; SPRITE_TEMP_SIZE];

        self.current_line = 241;
        self.cumulative_cpu_cyc = 0;

        self.fetch_scroll_x = 0;
        self.fetch_scroll_y = 0;
        self.current_scroll_x = 0;
        self.current_scroll_y = 0;

        self.is_dma_running = false;
        self.dma_cpu_src_addr = 0;
        self.dma_oam_dst_addr = 0;
    }
}

impl Ppu {

    fn run_dma(&mut self, system: &mut System, is_pre_transfer: bool) {
        debug_assert!(
            (!self.is_dma_running && is_pre_transfer) || (self.is_dma_running && !is_pre_transfer)
        );
        debug_assert!((self.dma_cpu_src_addr & 0x00ff) == 0x0000);

        let start_offset: u8 = if is_pre_transfer {
            0
        } else {
            OAM_DMA_COPY_SIZE_PER_PPU_STEP
        };
        let cpu_start_addr: u16 = self.dma_cpu_src_addr.wrapping_add(u16::from(start_offset));
        let oam_start_addr: u8 = self.dma_oam_dst_addr.wrapping_add(start_offset);

        let transfer_size: u16 = if is_pre_transfer {
            OAM_DMA_COPY_SIZE_PER_PPU_STEP as u16
        } else {
            (OAM_SIZE as u16) - u16::from(OAM_DMA_COPY_SIZE_PER_PPU_STEP)
        };

        for offset in 0..transfer_size {
            let cpu_addr = cpu_start_addr.wrapping_add(offset);
            let oam_addr = usize::from(oam_start_addr.wrapping_add(offset as u8));

            let cpu_data = system.read_u8(cpu_addr, false);
            self.oam[oam_addr] = cpu_data;
        }


        self.is_dma_running = is_pre_transfer;
    }

    fn draw_line(
        &mut self,
        system: &mut System,
        fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    ) {
        
        let nametable_base_addr = system.read_ppu_name_table_base_addr();
        let pattern_table_addr = system.read_ppu_bg_pattern_table_addr();
        let is_clip_bg_leftend = system.read_ppu_is_clip_bg_leftend();
        let is_write_bg = system.read_ppu_is_write_bg();
        let is_monochrome = system.read_is_monochrome();
        let master_bg_color = Color::from(system.video.read_u8(
            &mut system.rom,
            PALETTE_TABLE_BASE_ADDR + PALETTE_BG_OFFSET,
        ));

        let raw_y = self.current_line + u16::from(self.current_scroll_y);
        let offset_y = raw_y & 0x07; 
        let tile_base_y = raw_y >> 3; 
                                      
        let tile_global_y = tile_base_y % (SCREEN_TILE_HEIGHT * 2); 
        let tile_local_y = tile_global_y % SCREEN_TILE_HEIGHT; 
                                                               
        let is_nametable_position_top = tile_global_y < SCREEN_TILE_HEIGHT;

       
        let pixel_y = usize::from(self.current_line);
        for pixel_x in 0..VISIBLE_SCREEN_WIDTH {
            
            let (sprite_palette_data_back, sprite_palette_data_front) =
                self.get_sprite_draw_data(system, pixel_x, pixel_y);

            
            let offset_x = ((pixel_x as u16) + u16::from(self.current_scroll_x)) & 0x07;
            let tile_base_x = ((pixel_x as u16) + u16::from(self.current_scroll_x)) >> 3;
           
            let tile_global_x = tile_base_x % (SCREEN_TILE_WIDTH * 2);
            let tile_local_x = tile_global_x % SCREEN_TILE_WIDTH;
            let is_nametable_position_left = tile_global_x < SCREEN_TILE_WIDTH; 

            
            let target_nametable_base_addr = nametable_base_addr +
                (if is_nametable_position_left { 0x0000 } else { 0x0400 }) + 
                (if is_nametable_position_top  { 0x0000 } else { 0x0800 }); 

            let attribute_base_addr = target_nametable_base_addr + ATTRIBUTE_TABLE_OFFSET; 
            let attribute_x_offset = (tile_global_x >> 2) & 0x7;
            let attribute_y_offset = tile_global_y >> 2;
            let attribute_addr =
                attribute_base_addr + (attribute_y_offset << 3) + attribute_x_offset;

      
            let raw_attribute = system.video.read_u8(&mut system.rom, attribute_addr);
            let bg_palette_id = match (tile_local_x & 0x03 < 0x2, tile_local_y & 0x03 < 0x2) {
                (true, true) => (raw_attribute >> 0) & 0x03,  // top left
                (false, true) => (raw_attribute >> 2) & 0x03, // top right
                (true, false) => (raw_attribute >> 4) & 0x03, // bottom left
                (false, false) => (raw_attribute >> 6) & 0x03, // bottom right
            };

  
            let nametable_addr = target_nametable_base_addr + (tile_local_y << 5) + tile_local_x;
            let bg_tile_id = u16::from(system.video.read_u8(&mut system.rom, nametable_addr));

            let bg_pattern_table_base_addr = pattern_table_addr + (bg_tile_id << 4);
            let bg_pattern_table_addr_lower = bg_pattern_table_base_addr + offset_y;
            let bg_pattern_table_addr_upper = bg_pattern_table_addr_lower + 8;
            let bg_data_lower = system
                .video
                .read_u8(&mut system.rom, bg_pattern_table_addr_lower);
            let bg_data_upper = system
                .video
                .read_u8(&mut system.rom, bg_pattern_table_addr_upper);

            let bg_palette_offset = (((bg_data_upper >> (7 - offset_x)) & 0x01) << 1)
                | ((bg_data_lower >> (7 - offset_x)) & 0x01);
            let bg_palette_addr = (PALETTE_TABLE_BASE_ADDR + PALETTE_BG_OFFSET) +   
                (u16::from(bg_palette_id) << 2) + 
                u16::from(bg_palette_offset);

           
            let is_bg_clipping = is_clip_bg_leftend && (pixel_x < 8);
            let is_bg_tranparent = (bg_palette_addr & 0x03) == 0x00; 
            let bg_palette_data: Option<u8> = if is_bg_clipping || !is_write_bg || is_bg_tranparent
            {
                None
            } else {
                Some(system.video.read_u8(&mut system.rom, bg_palette_addr))
            };

         
            let mut draw_color = master_bg_color;

           
            'select_color: for palette_data in &[
                sprite_palette_data_front,
                bg_palette_data,
                sprite_palette_data_back,
            ] {
               
                if let Some(color_index) = palette_data {
                    let c = Color::from(*color_index);
                    draw_color = c;
                    break 'select_color;
                }
            }
       
            fb[pixel_y][pixel_x][0] = draw_color.0;
            fb[pixel_y][pixel_x][1] = draw_color.1;
            fb[pixel_y][pixel_x][2] = draw_color.2;

           
            if is_monochrome {
                let data = ((u16::from(fb[pixel_y][pixel_x][0])
                    + u16::from(fb[pixel_y][pixel_x][1])
                    + u16::from(fb[pixel_y][pixel_x][2]))
                    / 3) as u8;
                fb[pixel_y][pixel_x][0] = data;
                fb[pixel_y][pixel_x][1] = data;
                fb[pixel_y][pixel_x][2] = data;
            }
        }
    }

    fn get_sprite_draw_data(
        &mut self,
        system: &mut System,
        pixel_x: usize,
        pixel_y: usize,
    ) -> (Option<u8>, Option<u8>) {
   
        if !system.read_ppu_is_write_sprite() {
            return (None, None);
        }
     
        let mut sprite_palette_data_back: Option<u8> = None; 
        let mut sprite_palette_data_front: Option<u8> = None; 
        'draw_sprite: for &s in self.sprite_temps.iter() {
            if let Some(sprite) = s {

                let sprite_x = usize::from(sprite.x);
                let sprite_y = usize::from(sprite.y);
               
                let is_sprite_clipping = system.read_ppu_is_clip_sprite_leftend() && (pixel_x < 8);
                
                if !is_sprite_clipping
                    && (sprite_x <= pixel_x)
                    && (pixel_x < usize::from(sprite_x + SPRITE_WIDTH))
                {
                   
                    let sprite_offset_x: usize = pixel_x - sprite_x; 
                    let sprite_offset_y: usize = pixel_y - sprite_y - 1; 
                    debug_assert!(sprite_offset_x < SPRITE_WIDTH);
                    debug_assert!(sprite_offset_y < usize::from(system.read_ppu_sprite_height()));
                   
                    let (sprite_pattern_table_addr, sprite_tile_id): (u16, u8) = match sprite
                        .tile_id
                    {
                        TileId::Normal { id } => (system.read_ppu_sprite_pattern_table_addr(), id),
                       
                        TileId::Large {
                            pattern_table_addr,
                            upper_tile_id,
                            lower_tile_id,
                        } => {
                            let is_upper = sprite_offset_y < SPRITE_NORMAL_HEIGHT;
                            let is_vflip = sprite.attr.is_vert_flip; 
                            let id = match (is_upper, is_vflip) {
                                (true, false) => upper_tile_id,  
                                (false, false) => lower_tile_id, 
                                (true, true) => lower_tile_id,   
                                (false, true) => upper_tile_id,  
                            };
                            (pattern_table_addr, id)
                        }
                    };
                    
                    let tile_offset_x: usize = if !sprite.attr.is_hor_flip {
                        sprite_offset_x
                    } else {
                        SPRITE_WIDTH - 1 - sprite_offset_x
                    };
                    let tile_offset_y: usize = if !sprite.attr.is_vert_flip {
                        sprite_offset_y % SPRITE_NORMAL_HEIGHT
                    } else {
                        SPRITE_NORMAL_HEIGHT - 1 - (sprite_offset_y % SPRITE_NORMAL_HEIGHT)
                    };
               
                    let sprite_pattern_table_base_addr = u16::from(sprite_pattern_table_addr)
                        + (u16::from(sprite_tile_id) * PATTERN_TABLE_ENTRY_BYTE);
                    let sprite_pattern_table_addr_lower =
                        sprite_pattern_table_base_addr + (tile_offset_y as u16);
                    let sprite_pattern_table_addr_upper = sprite_pattern_table_addr_lower + 8;
                    let sprite_data_lower = system
                        .video
                        .read_u8(&mut system.rom, sprite_pattern_table_addr_lower);
                    let sprite_data_upper = system
                        .video
                        .read_u8(&mut system.rom, sprite_pattern_table_addr_upper);
                   
                    let sprite_palette_offset =
                        (((sprite_data_upper >> (7 - tile_offset_x)) & 0x01) << 1)
                            | ((sprite_data_lower >> (7 - tile_offset_x)) & 0x01);
                 
                    let sprite_palette_addr = (PALETTE_TABLE_BASE_ADDR + PALETTE_SPRITE_OFFSET) +        
                        (u16::from(sprite.attr.palette_id) * PALETTE_ENTRY_SIZE) + 
                        u16::from(sprite_palette_offset); 
                                                          
                    let is_tranparent = (sprite_palette_addr & 0x03) == 0x00; 
                    if !is_tranparent {
                       
                        let sprite_palette_data = system
                            .video
                            .read_u8(&mut system.rom, sprite_palette_addr);
                       
                        if sprite.attr.is_draw_front {
                            sprite_palette_data_front = Some(sprite_palette_data);
                        } else {
                            sprite_palette_data_back = Some(sprite_palette_data);
                        }
                    }
                }
            } else {
             
                break 'draw_sprite;
            }
        }
     
        (sprite_palette_data_back, sprite_palette_data_front)
    }


 
    fn fetch_sprite(&mut self, system: &mut System) {
    
        if !system.read_ppu_is_write_sprite() {
            return;
        }
  
        let sprite_begin_y = self.current_line;
        let sprite_height = u16::from(system.read_ppu_sprite_height());
        let is_large = sprite_height == 16;
 
        self.sprite_temps = [None; SPRITE_TEMP_SIZE];

        let mut tmp_index = 0;
        'search_sprite: for sprite_index in 0..NUM_OF_SPRITE {
            let target_oam_addr = sprite_index << 2;
 
            let sprite_y = u16::from(self.oam[target_oam_addr]);
            let sprite_end_y = sprite_y + sprite_height;
     
            if (sprite_y < sprite_begin_y) && (sprite_begin_y <= sprite_end_y) {
          
                let is_zero_hit_delay = sprite_begin_y > (sprite_end_y - 3); 
                if sprite_index == 0 && is_zero_hit_delay {
                    system.write_ppu_is_hit_sprite0(true);
                }
              
                if tmp_index >= SPRITE_TEMP_SIZE {
                    system.write_ppu_is_sprite_overflow(true);
                    break 'search_sprite;
                } else {
                    debug_assert!(tmp_index < SPRITE_TEMP_SIZE);
                  
                    self.sprite_temps[tmp_index] = Some(Sprite::from(
                        is_large,
                        self.oam[target_oam_addr],
                        self.oam[target_oam_addr + 1],
                        self.oam[target_oam_addr + 2],
                        self.oam[target_oam_addr + 3],
                    ));
                    tmp_index = tmp_index + 1;
                }
            }
        }
    }


    fn update_line(
        &mut self,
        system: &mut System,
        fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    ) -> Option<Interrupt> {
      
        self.current_scroll_x = self.fetch_scroll_x;
        self.current_scroll_y = self.fetch_scroll_y;
 
        if self.is_dma_running {
          
            self.run_dma(system, false);
        }
        let (is_dma_req, dma_cpu_src_addr) = system.read_oam_dma();
        if is_dma_req {
           
            self.dma_cpu_src_addr = dma_cpu_src_addr;
            self.dma_oam_dst_addr = system.read_ppu_oam_addr();
            self.run_dma(system, true);
        }
       
        system.write_ppu_is_hit_sprite0(false);
        system.write_ppu_is_sprite_overflow(false);

     
        match LineStatus::from(self.current_line) {
            LineStatus::Visible => {
              
                self.fetch_sprite(system);
              
                self.draw_line(system, fb);
                
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;

                None
            }
            LineStatus::PostRender => {
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
                None
            }
            LineStatus::VerticalBlanking(is_first) => {
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
                if is_first {
                    system.write_ppu_is_vblank(true);
                }
                
                if system.read_ppu_nmi_enable() && system.read_ppu_is_vblank() {
                    Some(Interrupt::NMI)
                } else {
                    None
                }
            }
            LineStatus::PreRender => {
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
               
                system.write_ppu_is_vblank(false);

                None
            }
        }
    }

    pub fn step(
        &mut self,
        cpu_cyc: usize,
        system: &mut System,
        fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    ) -> Option<Interrupt> {
       
        let (_, scroll_x, scroll_y) = system.read_ppu_scroll();
        self.fetch_scroll_x = scroll_x;
        self.fetch_scroll_y = scroll_y;

       
        let (_, ppu_addr) = system.read_ppu_addr();
        let (is_read_ppu_req, is_write_ppu_req, ppu_data) = system.read_ppu_data();

        if is_write_ppu_req {
            system
                .video
                .write_u8(&mut system.rom, ppu_addr, ppu_data);
            system.increment_ppu_addr();
        }
        if is_read_ppu_req {
            let data = system.video.read_u8(&mut system.rom, ppu_addr);
            system.write_ppu_data(data);
            system.increment_ppu_addr();
        }

        
        let oam_addr = system.read_ppu_oam_addr();
        let (is_read_oam_req, is_write_oam_req, oam_data) = system.read_oam_data();
        if is_write_oam_req {
            self.oam[usize::from(oam_addr)] = oam_data;
        }
        if is_read_oam_req {
            let data = self.oam[usize::from(oam_addr)];
            system.write_oam_data(data);
        }

        
        let total_cyc = self.cumulative_cpu_cyc + cpu_cyc;
        if total_cyc >= CPU_CYCLE_PER_LINE {
            self.cumulative_cpu_cyc = total_cyc - CPU_CYCLE_PER_LINE;
            self.update_line(system, fb)
        } else {
            self.cumulative_cpu_cyc = total_cyc;
            None
        }
    }
}