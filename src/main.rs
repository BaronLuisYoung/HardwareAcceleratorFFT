#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use hal::delay::Delay;

use crate::hal::prelude::*;
use crate::rt::entry;
use crate::rt::ExceptionFrame;

#[entry]
fn main() -> ! {
    // Log a hello message using defmt
    defmt::info!("Hello from RTT!");

    // Take peripherals
    let cp = cortex_m::Peripherals::take().expect("Failed to take core peripherals");
    let dp = hal::stm32::Peripherals::take().expect("Failed to take device peripherals");

    // Configure PA0 as analog input (ADC1_IN5)
    let _adc_pin = Pin::new(Port::A, 0, PinMode::Analog);

    let clocks = rcc.cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr, &mut pwr);

    // Configure PA5 (LD2) as push-pull output
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut ld2 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // Configure PC13 (user button) as input with pull-up
    //let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    // let button = gpioc
    //     .pc13
    //     .into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);

    let mut timer = Delay::new(cp.SYST, clocks);
    // Check button state and control LD2
        loop {
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        ld2.set_high();
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        ld2.set_low();
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}