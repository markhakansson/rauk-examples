// cyccnt.rs

#![no_std]
#![no_main]

extern crate cortex_m;

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_klee as _;

#[no_mangle]
fn main() {
    let mut core = cortex_m::Peripherals::take().unwrap();
    // core.DCB.enable_trace();
    // core.DWT.enable_cycle_counter();

    let start: u32 = core.DWT.cyccnt.read();

    // some code to measure
    // ...

    let end = core.DWT.cyccnt.read();

    //let _time = end.wrapping_sub(start);
    let _time = end - start;
}
