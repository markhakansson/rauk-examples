#![no_main]
#![no_std]
#![feature(asm)]

#[cfg(feature = "klee-analysis")]
use panic_abort as _;
#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use cortex_m::asm;

    #[resources]
    struct Resources {
        a: u8,
        #[init(1337)]
        b: u16,
        c: u16,
    }

    #[init]
    fn init(_cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let value = 30;
        (init::LateResources { a: value, c: 50 }, init::Monotonics {})
    }

    #[task(binds = EXTI0, resources = [a])]
    fn increment(mut cx: increment::Context) {
        cx.resources.a.lock(|a| {
            *a + 1;
        });
    }

    #[task(binds = EXTI1, resources = [a, b])]
    fn decrement(mut cx: decrement::Context) {
        cx.resources.a.lock(|a| *a - 1);
        cx.resources.b.lock(|b| *b - 10);
    }

    #[task(resources = [c])]
    fn software_task(mut cx: software_task::Context) {
        cx.resources.c.lock(|c| *c + 50);
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        panic!("idle");
    }
}
