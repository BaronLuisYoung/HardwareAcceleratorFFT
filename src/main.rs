#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::delay::Delay;
use cortex_m_rt::entry; // The runtime
use hal::{
    self,
    clocks::{Clocks, InputSrc},
    gpio::{Pin, PinMode, Port},
    pac,
};

use defmt_rtt as _;

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Set up CPU peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up microcontroller peripherals
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    // Setup a delay, based on the Cortex-m systick.
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());
    // Port::A, 0 because the LED is described as PA0 
    let mut led = Pin::new(Port::A, 0, PinMode::Output);

    // Now, enjoy the lightshow!
    loop {
        defmt::debug!("Our demo is alive");
        led.set_low();
        delay.delay_ms(1_000);
        led.set_high();
        delay.delay_ms(1_000);
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}