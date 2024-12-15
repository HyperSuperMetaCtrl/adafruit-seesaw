#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{devices::NeoRotary4, prelude::*, SeesawRefCell, modules::quad_encoder::QuadEncoderModule};

use rppal::i2c::I2c;
use rppal::hal::Delay;

fn main() {
    let i2c = I2c::new().unwrap();
    let delay = Delay::new();
    let seesaw = SeesawRefCell::new(delay, i2c);
    let mut encoder = NeoRotary4::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start RotaryEncoder");


    loop {
        for i in 0..4 {
        let position = encoder.position(i).expect("Failed to get position");
        let c = color_wheel(((position & 0xFF) as u8).wrapping_mul(3));
        let Color(r, g, b) = c.set_brightness(255);
            println!("{}:{}",i, position);
        encoder
            .set_nth_neopixel_color(i as u16, r, g, b)
            .and_then(|_| encoder.sync_neopixel())
            .expect("Failed to set neopixel");

        if let Ok(true) = encoder.button(i) {
            println!("Button {} pressed",i);
        }
        }
    }
}


fn color_wheel(byte: u8) -> Color {
    match byte {
        0..=84 => Color(255 - byte * 3, 0, byte * 3),
        85..=169 => Color(0, (byte - 85) * 3, 255 - (byte - 85) * 3),
        _ => Color((byte - 170) * 3, 255 - (byte - 170) * 3, 0),
    }
}

struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn set_brightness(self, brightness: u8) -> Self {
        Self(
            ((self.0 as u16 * brightness as u16) >> 8) as u8,
            ((self.1 as u16 * brightness as u16) >> 8) as u8,
            ((self.2 as u16 * brightness as u16) >> 8) as u8,
        )
    }
}
