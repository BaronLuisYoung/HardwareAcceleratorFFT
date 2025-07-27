read sections:
$ arm-none-eabi-readelf -h -S  target/thumbv7em-none-eabihf/debug/stm32l476rg_blinky

list probes:
probe-rs list

stlink: https://probe.rs/docs/getting-started/probe-setup/
programming with cargo flash (uses stlink and must have win usb, source: https://github.com/probe-rs/probe-rs/issues/2848)

cargo flash --chip STM32L476RG --probe 0483:374b:066CFF544949878667255317 --connect-under-reset



The following debug probes were found:
[0]: STLink V2-1 -- 0483:374b:066CFF3031454D3043073828 (ST-LINK)


size -Ax target/thumbv7m-none-eabi/examples/app