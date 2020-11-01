use minifb::{Key, Scale, Window, WindowOptions};
use std::fmt;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

#[derive(Clone, Debug)]
struct Display {
    // vram
    pub vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],

    // update
    pub update: bool,

    // minifb buffer
    pub buffer: Vec<u32>,
}

impl Display {
    /// Create and return a new Display instance.
    pub fn new() -> Display {
        let buffer_width = CHIP8_WIDTH;
        let buffer_height = CHIP8_HEIGHT;
        Display {
            vram: [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
            buffer: vec![from_u8_rgb(0, 0, 0); buffer_height * buffer_width],
            update: true,
        }
    }

    pub fn draw(&mut self, xpos: usize, ypos: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        let h = sprite.len();

        for j in 0..h {
            for i in 0..8 {
                // screen wrap if necessary
                let y = (ypos + j) % CHIP8_HEIGHT;
                let x = (xpos + i) % CHIP8_WIDTH;
                // draw each sprite pixel with a XOR operation
                // i.e. toggle the pixel
                // 0x80 = 1000 0000 : allows to check each pixel in the sprite
                if (sprite[j] & (0x80 >> i)) != 0x00 {
                    if self.vram[y][x] == 0x01 {
                        collision = true;
                    }
                    self.vram[y][x] ^= 0x01;
                }
            }
        }
        self.update = true;

        collision
    }
}

impl fmt::Display for Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..CHIP8_WIDTH {
            for j in 0..CHIP8_HEIGHT {
                write!(f, "{}", self.vram[j][i])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn emulate() {
    let window_width = CHIP8_WIDTH;
    let window_height = CHIP8_HEIGHT;
    let buffer_width = CHIP8_WIDTH;
    let buffer_height = CHIP8_HEIGHT;

    let mut display = Display::new();
    let mut window = Window::new(
        "Test",
        window_width,
        window_height,
        WindowOptions {
            scale: Scale::X4,
            resize: true,
            borderless: false,
            title: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    display.draw(0, 0, &FONT_SET[0..5]);
    display.draw(5, 0, &FONT_SET[5..10]);
    display.draw(10, 0, &FONT_SET[15..20]);
    display.draw(15, 0, &FONT_SET[20..25]);
    display.draw(20, 0, &FONT_SET[25..30]);
    println!("{}", display);
    for y in 0u32..(buffer_height as u32) {
        for x in 0u32..(buffer_width as u32) {
            if display.vram[y as usize][x as usize] == 1u8 {
                let ve = y * (buffer_width as u32) + x;
                display.buffer[ve as usize] = from_u8_rgb(255, 0, 0);
            }
        }
    }

    window
        .update_with_buffer(&display.buffer, buffer_width, buffer_height)
        .unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
        if window.is_key_down(Key::A) {
            display.draw(20, 8, &FONT_SET[25..30]);
            for y in 0u32..(buffer_height as u32) {
                for x in 0u32..(buffer_width as u32) {
                    if display.vram[y as usize][x as usize] == 1u8 {
                        let ve = y * (buffer_width as u32) + x;
                        display.buffer[ve as usize] = from_u8_rgb(255, 0, 0);
                    }
                }
            }
            window
                .update_with_buffer(&display.buffer, buffer_width, buffer_height)
                .unwrap();
            println!("Key A is down");
        }
    }
}

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
