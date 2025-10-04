#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::cell::RefCell;

use critical_section::Mutex;
// --- IMPORTS CHANGED ---
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock, delay::Delay, gpio::{Event, Input, InputConfig, Io, Pull}, handler, main, ram, rmt::Rmt, time::{Instant, Rate}
};
use esp_println::println;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use led_effects::{controller::EffectController, policedot_effect::PoliceDot, policetrail_effect::PoliceTrail};
use led_effects::solid_effect::SolidColor;
use smart_leds::{RGB8, SmartLedsWrite};

extern crate alloc;
use alloc::boxed::Box;
// use alloc::vec::Vec; // No longer needed

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static EFFECT_CONTROLLER: Mutex<RefCell<Option<EffectController>>> = Mutex::new(RefCell::new(None));

const NUM_LEDS: usize = 16;

// --- MAIN FUNCTION CHANGED TO BLOCKING ---
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(handler);

    let button_pin = peripherals.GPIO9;
    let mut button = Input::new(button_pin, InputConfig::default().with_pull(Pull::Up));
    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button)
    });

    esp_alloc::heap_allocator!(size: 64 * 1024);

    println!("Setup done.\r");

    // --- RMT INITIALIZATION CHANGED TO BLOCKING ---
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        let frequency: Rate = Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT");

    let rmt_channel = rmt.channel0;

    // --- BUFFER AND LED ADAPTER CHANGED TO BLOCKING VERSIONS ---
    let rmt_buffer = smart_led_buffer!(NUM_LEDS);
    let mut led = SmartLedsAdapter::new(rmt_channel, peripherals.GPIO3, rmt_buffer);
    
    println!("LED Setup done.\r");

    let mut leds: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let mut last_update = Instant::now();

    let mut effect_controller = EffectController::new();
    effect_controller.add_effect(Box::new(SolidColor {
        color: RGB8::new(255, 0, 0),
    }));
    effect_controller.add_effect(Box::new(PoliceDot::new(1.0, 2, NUM_LEDS)));
    effect_controller.add_effect(Box::new(PoliceTrail::new(1.0, 2, 8, NUM_LEDS)));
    effect_controller.set_effect_by_name("PoliceTrail");
    
    let delay = Delay::new();

    // --- LOOP CHANGED TO USE BLOCKING CALLS ---
    loop {
        let now = Instant::now();
       let delta = (now - last_update).as_millis() as f32 / 1000.0;
        last_update = now;

        let current_effect = effect_controller.get_current_effect();
        current_effect.before_render(delta);

        for i in 0..NUM_LEDS {
            leds[i] = current_effect.render(i, NUM_LEDS);
        }

        // --- WRITE CALL IS NOW BLOCKING (NO .await) ---
        led.write(leds.iter().cloned()).unwrap();

        // --- DELAY IS NOW BLOCKING ---
        //delay.delay_ms(1000u32);
        delay.delay_millis(50u32);
    }
}

#[handler]
#[ram]
fn handler() {

        if critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .is_interrupt_set()
    }) {
       
    } else {

    }

    critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt()
    });
    
}