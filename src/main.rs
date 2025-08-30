#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::delay::Delay;
use core::panic::PanicInfo;

use hal::{
    adc::{Adc, AdcDevice, AdcConfig, ClockMode, SampleTime, Prescaler, OperationMode},
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    pac,
    prelude::*,
};

use defmt_rtt as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    // Set up clocks
    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    // Delay for LED blink
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    // LED on PA1
    let mut led = Pin::new(Port::A, 1, PinMode::Output);

    // PA0 as analog input (ADC1_IN5)
    let _adc_pin = Pin::new(Port::A, 0, PinMode::Analog);
    
    // ADC1 in one-shot mode
    let mut adc = Adc::new_adc1(
        dp.ADC1,
        AdcDevice::One,
        Default::default(),
        clock_cfg.systick(),
    )
    .unwrap();

    // enable the5 GPIOx_ASCR register
    let gpioa = &dp.GPIOA;
    let reg = gpioa.ascr();
    reg.write(|w| w.asc0().set_bit());


    loop {
        // Poll the ADC for channel 5 (PA0)
        let value: f32 = adc.read_voltage(5).unwrap();
        defmt::println!("ADC value: {}", value);

        // Blink LED
        led.set_low();
        delay.delay_ms(250);
        led.set_high();
        delay.delay_ms(250);
    }
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
