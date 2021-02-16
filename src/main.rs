//! main.rs

#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_klee as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true)]
mod app {
    #[resources]
    struct Resources {
        a: u8,
        #[init(10)]
        b: u16,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let value = 30;
        init::LateResources { a: value }
    }

    #[task(binds = EXTI0, resources = [a])]
    fn increment(mut cx: increment::Context) {
        cx.resources.a.lock(|a| *a + 1);
    }

    #[task(binds = EXTI1, resources = [a, b])]
    fn decrement(mut cx: decrement::Context) {
        cx.resources.a.lock(|a| *a - 1);
        cx.resources.b.lock(|b| *b - 10);
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        panic!("idle");
    }
}
