

#[derive(Copy, Clone)]
pub enum PadButton {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Pad {
    pub button_reg: u8,
    pub read_shift_index: u8,
    pub strobe_enable: bool,
}
impl Pad {
    fn default() -> Self {
        Self {
            button_reg: 0,
            read_shift_index: 0,
            strobe_enable: false,
        }
    }
}

impl EmulateControl for Pad {
    fn reset(&mut self) {
        self.button_reg = 0;
        self.read_shift_index = 0;
        self.strobe_enable = false;
    }
}

impl Pad {
    pub fn write_strobe(&mut self, is_enable: bool) {
        self.strobe_enable = is_enable;
        if is_enable {
            self.read_shift_index = 0;
        }
    }

    pub fn read_out(&mut self) -> u8 {
        let data = self.button_reg.wrapping_shr(self.read_shift_index.into()) & 0x01;
        if !self.strobe_enable {
            self.read_shift_index = (self.read_shift_index + 1) % 8;
        }
        data
    }
    pub fn push_button(&mut self, button: PadButton) {
        match button {
            PadButton::A => self.button_reg = self.button_reg | 0x01u8,
            PadButton::B => self.button_reg = self.button_reg | 0x02u8,
            PadButton::Select => self.button_reg = self.button_reg | 0x04u8,
            PadButton::Start => self.button_reg = self.button_reg | 0x08u8,
            PadButton::Up => self.button_reg = self.button_reg | 0x10u8,
            PadButton::Down => self.button_reg = self.button_reg | 0x20u8,
            PadButton::Left => self.button_reg = self.button_reg | 0x40u8,
            PadButton::Right => self.button_reg = self.button_reg | 0x80u8,
        }
    }
    pub fn release_button(&mut self, button: PadButton) {
        match button {
            PadButton::A => self.button_reg = self.button_reg & (!0x01u8),
            PadButton::B => self.button_reg = self.button_reg & (!0x02u8),
            PadButton::Select => self.button_reg = self.button_reg & (!0x04u8),
            PadButton::Start => self.button_reg = self.button_reg & (!0x08u8),
            PadButton::Up => self.button_reg = self.button_reg & (!0x10u8),
            PadButton::Down => self.button_reg = self.button_reg & (!0x20u8),
            PadButton::Left => self.button_reg = self.button_reg & (!0x40u8),
            PadButton::Right => self.button_reg = self.button_reg & (!0x80u8),
        }
    }
}