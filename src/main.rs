extern crate minifb;
mod core;

fn main() {
    core::display::emulate();
    println!("Chip8 Emulator");
}
