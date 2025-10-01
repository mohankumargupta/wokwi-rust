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
use esp_hal::{clock::CpuClock, time::Instant};
use esp_hal::timer::systimer::SystemTimer;
use esp_println::println;
use esp_hal::{rmt::Rmt, time::Rate};
use esp_hal_smartled::{SmartLedsAdapterAsync, buffer_size_async};
//use log::info;
use smart_leds::{
    RGB8, SmartLedsWriteAsync, brightness, gamma,
    hsv::{Hsv, hsv2rgb}
};
use led_effects::controller::EffectController;
use led_effects::solid_effect::SolidColor;



extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const NUM_LEDS: usize = 4;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    println!("Setup done.\r");


        // Configure RMT (Remote Control Transceiver) peripheral globally
    // <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/rmt.html>
    let rmt: Rmt<'_, esp_hal::Async> = {
        let frequency: Rate =  Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT")
    .into_async();

    // We use one of the RMT channels to instantiate a `SmartLedsAdapterAsync` which can
    // be used directly with all `smart_led` implementations
    let rmt_channel = rmt.channel0;
    let rmt_buffer = [0_u32; buffer_size_async(4)];

    let mut led: SmartLedsAdapterAsync<_, 100> =  SmartLedsAdapterAsync::new(rmt_channel, peripherals.GPIO3, rmt_buffer);


    println!("LED Setup done.\r");

    let mut leds: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let mut last_update = Instant::now();

        // -- Create and populate the EffectController --
    let mut effect_controller = EffectController::new();
    effect_controller.add_effect(Box::new(SolidColor { color: RGB8::new(255, 0, 0) }));

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        let now = Instant::now();
        let delta: f32 = 0.0;
        last_update = now;

        let current_effect = effect_controller.get_current_effect();
        current_effect.before_render(delta);

        // -- render --
        for i in 0..NUM_LEDS {
            leds[i] = current_effect.render(i, NUM_LEDS);
        }


        led.write(brightness(leds.iter().cloned(), 255)).await.unwrap();
        
        
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
