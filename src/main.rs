//! Uses the user button (PC13) to turn LD2 (PA5) on/off on an STM32L4 Nucleo board

#![no_std]
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
    // Take peripherals
    let cp = cortex_m::Peripherals::take().expect("Failed to take core peripherals");
    let dp = hal::stm32::Peripherals::take().expect("Failed to take device peripherals");

    // Configure clocks
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

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

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault: {:#?}", ef);
}