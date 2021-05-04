#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_klee as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use cortex_m::asm;
    use stm32f4xx_hal::{gpio::*, prelude::*};

    #[resources]
    struct Resources {
        a: u8,
        #[init(1337)]
        b: u16,
        c: u16,
        #[init([0; 1])]
        buffer: [u32; 1],
        dwt: stm32f4xx_hal::stm32::DWT,
    }

    #[init]
    fn init(mut cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let value = 30;
        (
            init::LateResources {
                a: value,
                c: 50,
                dwt: cx.core.DWT,
            },
            init::Monotonics {},
        )
    }

    #[no_mangle]
    #[task(binds = EXTI0, resources = [a, c])]
    fn increment(mut cx: increment::Context) {
        let mut a = cx.resources.a;
        let mut c = cx.resources.c;

        a.lock(|a| {
            c.lock(|c| {
                *a + 1;
                *c + 100;
            });
        });
    }

    #[task(binds = EXTI1, resources = [a, b])]
    fn decrement(mut cx: decrement::Context) {
        cx.resources.a.lock(|a| *a - 1);
        cx.resources.b.lock(|b| *b - 10);
    }

    #[task(binds = EXTI2, resources = [dwt, buffer])]
    fn reading(mut cx: reading::Context) {
        let start: u32 = cx.resources.dwt.lock(|dwt| dwt.cyccnt.read());
        let end: u32 = cx.resources.dwt.lock(|dwt| dwt.cyccnt.read());
        if end == 1234 {
            asm::delay(10_000);
            let _x: u32 = cx.resources.dwt.lock(|dwt| dwt.cyccnt.read());
        };
        let time = end - start;
        cx.resources.buffer.lock(|buffer| buffer[0] = time);
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        panic!("idle");
    }
}
