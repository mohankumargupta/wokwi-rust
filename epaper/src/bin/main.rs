#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
//use embassy_time::{Duration, Timer};
use embedded_graphics::{
    // mono_font::MonoTextStyleBuilder,
    // pixelcolor::BinaryColor::On as Black,
    draw_target::DrawTargetExt,
    pixelcolor::BinaryColor::{self, Off as White, On as Black},
    prelude::*,
    primitives::{circle, Circle, Line, PrimitiveStyleBuilder, Sector},
    //text::{Baseline, Text, TextStyleBuilder},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use epd_waveshare::{epd2in9_v2::*, graphics::DisplayRotation, prelude::*};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    spi::{
        master::{Config, Spi},
        Mode,
    },
    time::Rate,
    timer::timg::TimerGroup,
};
use log::info;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!\r\n");

    //SPI (blocking)
    let sclk = peripherals.GPIO18;
    let mosi = peripherals.GPIO23;
    let cs = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default()); //peripherals.GPIO5;

    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi);

    //epaper setup
    //    let busy = Input::new(peripherals.GPIO4, InputConfig::default());
    let busy = Input::new(
        peripherals.GPIO4,
        InputConfig::default().with_pull(Pull::Up),
    ); //peripherals.GPIO4;
    let mut reset = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default()); //peripherals.GPIO21;
    let dc = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default()); //peripherals.GPIO22;

    let mut delay = Delay::new();
    let mut spi_dev = ExclusiveDevice::new(spi, cs, delay).expect("Failed to create SPI device");

    info!("SPI initialized\r\n");
    // Check BUSY pin after reset

    // First, perform a proper hardware reset
    reset.set_low();
    delay.delay_millis(10); // hold low for at least 10ms
    reset.set_high();
    delay.delay_millis(200); // wait 200ms for power-up

    // while busy.is_low() {
    //     delay.delay_millis(10);
    // }
    info!("BUSY pin is HIGH after reset, proceeding with initialization\r\n");
    // Check BUSY pin
    let is_busy_low = busy.is_low();
    info!(
        "BUSY pin state: {}\r\n",
        if is_busy_low { "LOW" } else { "HIGH" }
    );

    let mut epd =
        Epd2in9::new(&mut spi_dev, busy, dc, reset, &mut delay, None).expect("EPD init failed");

    let mut display = Display2in9::default();
    display.set_rotation(DisplayRotation::Rotate90);


    info!("Clearing display\r\n");
    let _ = display.clear(White.into());

    let mut shifted_display = display.translated(Point::new(0, -12));

    // let style = PrimitiveStyleBuilder::new()
    //     .stroke_color(Color::Black)
    //     .stroke_width(3)
    //     .fill_color(Color::Black)
    //     .build();

    // let circle_diameter = 100;
    // let circle_radius = circle_diameter / 2;
    // Sector::new(
    //     Point::new(width / 2 - circle_radius, -40 + height / 2),
    //     circle_diameter as u32,
    //     0.0.deg(),
    //     -315.0.deg(),
    // )
    // .into_styled(style)
    // .draw(&mut shifted_display)
    // .unwrap();


    
    //let _ = epd.sleep(&mut spi_dev, &mut delay);

    // main loop

    let _ = spawner;

    let app = App::new();
    let bbox = &shifted_display.bounding_box();
    let width = bbox.size.width as i32;
    let height = bbox.size.height as i32;
    app.draw_pie(&mut shifted_display, width, height);
    let _ = epd.update_and_display_frame(&mut spi_dev, &display.buffer(), &mut delay);
    
    loop {
        info!("Hello world!\r\n");
        //delay.delay_millis(10);
        //Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

struct App {
    pie: bool,
}

impl App {
    pub fn new() -> Self {
        Self { pie: true }
    }

    pub fn draw_pie<D>(self, target: &mut D, width: i32, height: i32)
    where
        D: DrawTarget<Color = Color>,
    {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Color::Black)
            .stroke_width(3)
            .fill_color(Color::Black)
            .build();

        let circle_diameter = 100;
        let circle_radius = circle_diameter / 2;

        let _ = Sector::new(
            Point::new(width / 2 - circle_radius, -40 + height / 2),
            circle_diameter as u32,
            0.0.deg(),
            -315.0.deg(),
        )
        .into_styled(style)
        .draw(target);
    }

    pub fn draw_gag() {}
}
