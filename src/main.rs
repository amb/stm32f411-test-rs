#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::{Error, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals, Config};
use embassy_time::Timer;
use embedded_graphics::{
    mono_font::{ascii::FONT_7X13_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    
    // let mut config = Config::default();
    // {
    //     use embassy_stm32::rcc::*;
    //     config.rcc.hse = Some(Hse {
    //         freq: Hertz(8_000_000),
    //         mode: HseMode::Bypass,
    //     });
    //     config.rcc.pll_src = PllSource::HSE;
    //     config.rcc.pll = Some(Pll {
    //         prediv: PllPreDiv::DIV4,
    //         mul: PllMul::MUL168,
    //         divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 168 / 2 = 168Mhz.
    //         divq: Some(PllQDiv::DIV7), // 8mhz / 4 * 168 / 7 = 48Mhz.
    //         divr: None,
    //     });
    //     config.rcc.ahb_pre = AHBPrescaler::DIV1;
    //     config.rcc.apb1_pre = APBPrescaler::DIV4;
    //     config.rcc.apb2_pre = APBPrescaler::DIV2;
    //     config.rcc.sys = Sysclk::PLL1_P;
    // }
    // let p = embassy_stm32::init(config);
        
    let p = embassy_stm32::init(Default::default());
    info!("Program started.");
    
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();
    display.set_brightness(Brightness::DIMMEST).unwrap();
    display.init().unwrap();

    let style = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
    let mut text = Text::new("Hello Rust!", Point::new(0, 0), style);

    let box_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::Off)
        .build();

    let mut x = 0 as i32;
    let mut y = 13 as i32;
    let mut xs = 1 as i32;
    let mut ys = 1 as i32;

    info!("toggle");
    led.set_low();
    loop {
        // Timer::after_millis(1).await;
        // Timer::after_micros(300_000).await;

        text.translate(Point::new(x, y)).draw(&mut display).unwrap();

        display.flush().unwrap();

        Rectangle::new(Point::new(x, y - 13), Size::new(7 * 11, 14))
            .into_styled(box_style)
            .draw(&mut display)
            .unwrap();

        x += xs;
        y += ys;

        if x < 0 || x >= 128 - 7 * 11 {
            xs = -xs;
            x += xs;
        }

        if y < 13 || y >= 64 {
            ys = -ys;
            y += ys;
        }
    }
}
