#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_rauk as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true)]
mod app {
    #[resources]
    struct Resources {
        a: u8,
    }
    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let value = 30;
        (init::LateResources { a: value }, init::Monotonics {})
    }

    #[task(binds = EXTI0, resources = [a])]
    fn increment(mut cx: increment::Context) {
        cx.resources.a.lock(|a| *a + 1);
    }

    #[task(binds = EXTI1, resources = [a])]
    fn decrement(mut cx: decrement::Context) {
        cx.resources.a.lock(|a| *a - 1);
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        panic!("idle");
    }
}
