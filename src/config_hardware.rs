

// //create function to configure operational amplifier
//     pub fn config_opamp1(dp: &hal::stm32::Peripherals) {
//          // Enable GPIOA for PA0 and PA3 before constraining RCC
//         dp.RCC.ahb2enr.modify(|_, w| w.gpioaen().set_bit());
        
//         //  Enable OPAMP peripheral clock (this allows register access)
//         dp.RCC.apb1enr1.modify(|_, w| w.opampen().set_bit());

//         //set PA0 and PA3 to analog mode
//         dp.GPIOA.moder.modify(|_, w| {
//             w.moder0().analog(); // PA0
//             w.moder3().analog()  // PA3
//         });

//         /* 
//         Configure operational amplifier OPAMP1
//         Enable OPAMP1 and configure its mode using raw bits (replace with correct field methods if available in PAC)
//         */
//         dp.OPAMP.opamp1_csr.modify(|r, w| unsafe {
//         let mut bits = r.bits();
//             bits &= !(0b11 << 2);      // Clear OPAMODE
//             bits |= 0b11 << 2;         // Follower mode
//             bits &= !(0b11 << 10);     // Clear VP_SEL
//             bits |= 0b00 << 10;        // VP_SEL = 0b00 = PA0
//         w.bits(bits)
//         });

//         /*Enable OPAMP when ready */
//         dp.OPAMP.opamp1_csr.modify(|r, w| unsafe {
//         w.bits(r.bits() | (1 << 0)) // Set OPAEN
//         });
// }



// Update the import to use the correct PAC path for stm32l476
use stm32l4xx_hal::stm32::{
    ADC1, ADC_COMMON, GPIOA, RCC, SYSCFG,
};

pub fn adc_wakeup(adc1: &hal::pac::adc1::RegisterBlock) {
    // If DEEPPWD is set, clear it
    if adc1.cr.read().deeppwd().bit_is_set() {
        adc1.cr.modify(|_, w| w.deeppwd().clear_bit());
    }

    // Enable internal voltage regulator
    adc1.cr.modify(|_, w| w.advregen().set_bit());

    // Wait for 20us with 80MHz sysclk
    let mut wait_time = 20 * (80_000_000 / 1_000_000);
    while wait_time > 0 {
        wait_time -= 1;
    }
}

pub fn adc_common_configuration() {
    let rcc = unsafe { &*RCC::ptr() };
    let syscfg = unsafe { &*SYSCFG::ptr() };
    let common = unsafe { &*ADC_COMMON::ptr() };

    // Enable SYSCFG clock
    rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    // Boost analog switch voltage
    syscfg.cfgr1.modify(|_, w| w.boosten().set_bit());

    // Enable VREFINT
    common.ccr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 22)) }); // VREFEN

    // No prescaler
    common.ccr.modify(|r, w| unsafe { w.bits(r.bits() & !(0b1111 << 18)) }); // PRESC = 0b0000

    // HCLK / 1 synchronous clock mode
    common.ccr.modify(|r, w| {
        let cleared = r.bits() & !(0b11 << 16); // Clear CKMODE
        unsafe { w.bits(cleared | (0b01 << 16)) } // CKMODE = 01
    });

    // Independent ADC mode
    common.ccr.modify(|r, w| unsafe { w.bits(r.bits() & !(0b11111)) });
}

pub fn adc_pin_init() {
    let gpioa = unsafe { &*GPIOA::ptr() };
    let rcc = unsafe { &*RCC::ptr() };

    rcc.ahb2enr.modify(|_, w| w.gpioaen().set_bit());

    // Set PA1 to analog mode
    gpioa.moder.modify(|r, w| {
        let cleared = r.bits() & !(0b11 << (1 * 2));
        unsafe { w.bits(cleared | (0b11 << (1 * 2))) }
    });

    // No pull-up/pull-down
    gpioa.pupdr.modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << (1 * 2))) });

    // Connect PA1 to analog switch
    gpioa.ascr.modify(|_, w| w.asc1().set_bit());
}

pub fn adc_init() {
    let adc1 = unsafe { &*ADC1::ptr() };
    let rcc = unsafe { &*RCC::ptr() };

    // Enable and reset ADC clock
    rcc.ahb2enr.modify(|_, w| w.adcen().set_bit());
    rcc.ahb2rstr.modify(|_, w| w.adcrst().set_bit());
    rcc.ahb2rstr.modify(|_, w| w.adcrst().clear_bit());

    // Disable ADC
    adc1.cr.modify(|_, w| w.aden().clear_bit());

    // Init sequence
    adc_pin_init();
    adc_common_configuration();
    adc_wakeup(adc1);

    // 12-bit resolution, right-aligned
    adc1.cfgr.modify(|_, w| {
        w.align().clear_bit(); // right alignment
        unsafe { w.res().bits(0b00) } // 12-bit
    });

    // Set sequence length to 1
    adc1.sqr1.modify(|r, w| {
        let mut val = r.bits();
        val &= !(0b1111 << 0); // clear L bits
        val &= !(0x1F << 6); // clear SQ1 bits
        val |= (6 << 6); // channel 6
        unsafe { w.bits(val) }
    });

    // Single-ended mode for channel 6
    adc1.difsel.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 6)) });

    // Sampling time: 24.5 cycles for channel 6
    adc1.smpr1.modify(|r, w| {
        let cleared = r.bits() & !(0b111 << 18); // clear SMP6
        unsafe { w.bits(cleared | (0b011 << 18)) } // SMP6 = 011
    });

    // Single conversion mode
    adc1.cfgr.modify(|_, w| {
        w.cont().clear_bit();
        unsafe { w.exten().bits(0b00) }
    });

    // Enable ADC and wait for ready
    adc1.cr.modify(|_, w| w.aden().set_bit());
    while adc1.isr.read().adrdy().bit_is_clear() {}
}


