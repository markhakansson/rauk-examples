//! main.rs

#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_klee as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true)]
mod app {
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let start: u32 = cx.core.DWT.cyccnt.read();
        let end = cx.core.DWT.cyccnt.read();
        let _time = end - start;
        init::LateResources {}
    }
}
