# can2040-sys

Raw Rust bindings to the can2040 c lib, which provides a CAN controller for the rp2040 MCU using its PIO state machine.

## Build

Requires Cmake >=3.13 to build

the can2040 library will be built from source automatically.

## Platforms

As can2040 is designed to only work on a rp2040, this 
crate will only target "thumbv6m-none-eabi". Attempting to build
for any other target will incur cmake build errors with the CMSIS includes.