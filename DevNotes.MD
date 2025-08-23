## STM32L476RG Blinky Development Notes

### ELF & Section Info
```sh
arm-none-eabi-readelf -h -S target/thumbv7em-none-eabihf/debug/stm32l476rg_blinky
```

### Probe Discovery
```sh
probe-rs list
```
**Detected Probes:**
- `[0]: STLink V2-1 -- 0483:374b:066CFF3031454D3043073828 (ST-LINK)`

### Programming & Flashing

- [ST-Link Setup Guide](https://probe.rs/docs/getting-started/probe-setup/)
- Programming with `cargo flash` (requires ST-Link and WinUSB):  
    [Probe-rs Issue #2848](https://github.com/probe-rs/probe-rs/issues/2848)

```sh
cargo flash --chip STM32L476RG --probe 0483:374b:066CFF3031454D3043073828 --connect-under-reset
```

### Binary Size
```sh
size -Ax target/thumbv7m-none-eabi/examples/app
```

### Reference Manual
- [STM32L476 Reference Manual (DM00083560)](https://www.st.com/resource/en/reference_manual/DM00083560.pdf)

### OPAMP1 Pin Mapping

| Signal      | Pin | Internal    | Comment                                          |
|-------------|-----|-------------|--------------------------------------------------|
| OPAMP1_VINM | PA1 | OPAMP1_OUT/PGA | Controlled by OPAMODE and VM_SEL bits           |
| OPAMP1_VINP | PA0 | DAC1_OUT1      | Controlled by VP_SEL bit                        |
| OPAMP1_VOUT | PA3 | ADC1_IN8/ADC2_IN8 | Connected when OPAMP enabled; ADC input controlled by ADC |

---

### Debugging

#### Start GDB with ELF Binary
```sh
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/stm32l476rg_blinky
```

#### OpenOCD Commands
```sh
openocd -f interface/stlink.cfg -f target/stm32l4x.cfg
openocd -f interface/stlink.cfg -f target/stm32l4x.cfg -c "arm semihosting enable"
```

#### GDB Workflow

1. **Load ELF (if not already loaded):**
        ```gdb
        file target/thumbv7em-none-eabihf/debug/stm32l476rg_blinky
        ```
2. **Connect to OpenOCD:**
        ```gdb
        target remote :3333
        ```
3. **Reset & Halt MCU (optional):**
        ```gdb
        monitor reset halt
        ```
4. **Set Breakpoint (optional):**
        ```gdb
        break main
        ```
5. **Flash Firmware (optional):**
        ```gdb
        load
        ```
6. **Start Execution:**
        ```gdb
        continue
        ```




$ cargo embed




    let mut vref = adc.enable_vref(&mut delay);
    let vref_raw = adc.read(&mut vref).unwrap();
    let vdda_mv = (VDDA_CALIB_MV * VrefCal::get().read() as u32) / vref_raw as u32;
    rprintln!("Measured VDDA: {} mV", vdda_mv);



    cargo doc --open