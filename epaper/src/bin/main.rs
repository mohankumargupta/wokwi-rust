#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::Rate,

};
use log::info;
use epd_waveshare::{epd2in9_v2::*, graphics::DisplayRotation, prelude::*};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    // --- SPI setup (blocking) ---
    let sclk = peripherals.GPIO18;
    let mosi = peripherals.GPIO23;
    let cs = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());//peripherals.GPIO5;

    let mut spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi);
    // note: no `.into_async()`, no `.with_cs()`

    info!("Embassy initialized!");

    // --- EPD pins ---
    let busy = Input::new(peripherals.GPIO4, InputConfig::default());//peripherals.GPIO4;
    let reset = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());//peripherals.GPIO21;
    let dc = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default());//peripherals.GPIO22;

    let mut delay = Delay::default();

    let mut epd = Epd2in9::new(&mut spi, busy,cs,  dc, reset, &mut delay)
       .expect("EPD init failed");

    // --- Loop ---
    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}
