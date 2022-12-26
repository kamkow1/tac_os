use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const ASCII_PRINTABLE_MIN: u8 = 0x20;
const ASCII_PRINTABLE_MAX: u8 = 0x7e;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[allow(dead_code)]
impl ScreenChar {
    fn write(&mut self, replacement: ScreenChar) {
        *self = replacement;
    }

    fn read(&self) -> Self {
        *self
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_cursor: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let charctr = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(charctr);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_cursor = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank_char = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank_char);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_cursor >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_cursor;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });

                self.column_cursor += 1;
            },
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                ASCII_PRINTABLE_MIN..=ASCII_PRINTABLE_MAX | b'\n' => self.write_byte(byte),
                _ => self.write_byte(ASCII_PRINTABLE_MAX),
            }
        }
    }

    pub fn set_cursor_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

// define a global interface
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_cursor: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

