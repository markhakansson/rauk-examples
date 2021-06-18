#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_rauk as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use cortex_m::asm;
    use stm32f4xx_hal::stm32::DWT;
    #[resources]
    struct Resources {
        dwt: DWT,
        #[init(0)]
        var: u8,
    }

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let dwt = cx.core.DWT;
        (init::LateResources { dwt }, init::Monotonics {})
    }

    #[task(binds = EXTI15_10, resources = [dwt, var])]
    fn task(mut cx: task::Context) {
        let start: u32 = cx.resources.dwt.lock(|dwt| dwt.cyccnt.read());
        let end: u32 = cx.resources.dwt.lock(|dwt| dwt.cyccnt.read());
        let time = end - start;

        if time == 12345 {
            asm::delay(100_000);
            cx.resources.var.lock(|var| *var += 1);
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }
}
