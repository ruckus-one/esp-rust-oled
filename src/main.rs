use anyhow::Ok;
use esp_idf_hal::i2c::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
    primitives::{Circle, PrimitiveStyleBuilder}
};

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let scl = peripherals.pins.gpio3;
    let sda = peripherals.pins.gpio2;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let off = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::Off)
        .build();

    let on = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let mut i = 0;
    let mut dir = 1;

    Text::with_baseline("Hello world!", Point::new(30, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust!!", Point::new(30, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    loop {
        Circle::new(Point::new(i, 40), 16)
            .into_styled(off)
            .draw(&mut display)
            .unwrap();

        if i > 100 { 
            dir = -1; 
        } 

        if i == 0 {
            dir = 1;
        }

        i += dir << 4;

        Circle::new(Point::new(i, 40), 16)
            .into_styled(on)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        FreeRtos::delay_ms(100);
    }

    Ok(())
}