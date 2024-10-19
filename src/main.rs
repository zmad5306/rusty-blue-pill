#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
use panic_halt as _; // Panic handler
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialize RTT for debugging
    rtt_init_print!();

    // Print an RTT debug message
    rprintln!("Starting Blinky on Blue Pill");
    
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Set up the clock. By default, it'll run at 8 MHz
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up the GPIO pin for the LED
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Set up a delay using the system timer
    let mut delay = Timer::syst(cp.SYST, &clocks).delay();

    loop {
        led.set_high(); // Turn on LED
        delay.delay_ms(1000_u16); // Wait for 1 second

        led.set_low(); // Turn off LED
        delay.delay_ms(1000_u16); // Wait for 1 second
    }
}
