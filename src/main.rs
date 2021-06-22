#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_rauk as _;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1, USART2])]
mod app {
    // This import should be included in klee feature inside rtic lib
    use cortex_m::asm;
    use stm32f4xx_hal::{
        gpio::*,
        prelude::*,
        stm32,
        timer::{Event, Timer},
    };

    #[resources]
    struct Resources {
        #[init(false)]
        led_on: bool,
        shared_u8: u8,
        #[init(1337)]
        shared_u16: u16,
        led: gpioa::PA5<Output<PushPull>>,
        timer: Timer<stm32::TIM2>,
        button: gpioc::PC13<Input<PullDown>>,
    }

    #[init]
    fn init(mut cx: init::Context) -> (init::LateResources, init::Monotonics) {
        // Setup clocks
        let rcc = cx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .hclk(8.mhz())
            .use_hse(8.mhz())
            .sysclk(8.mhz())
            .freeze();

        let mut syscfg = cx.device.SYSCFG.constrain();

        // Initialize LED output
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        let mut exti = cx.device.EXTI;

        // enable cycle counter
        unsafe {
            cx.core.DCB.enable_trace();
            cx.core.DWT.enable_cycle_counter();
            cx.core.DWT.cyccnt.write(0);
        }

        // Initialize B1 button
        let gpioc = cx.device.GPIOC.split();
        let mut button = gpioc.pc13.into_pull_down_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut exti);
        button.trigger_on_edge(&mut exti, Edge::RISING);

        // Setup timer (wraparound each 1 hertz)
        let timeout = 1.hz();
        let mut timer = Timer::tim2(cx.device.TIM2, timeout, clocks);
        timer.listen(Event::TimeOut);

        let value = 30;
        (
            init::LateResources {
                shared_u8: value,
                led,
                timer,
                button,
            },
            init::Monotonics {},
        )
    }

    // Aperiodic task that does some work everytime the button is pressed
    #[task(binds = EXTI15_10, priority = 2, resources = [button, shared_u8, shared_u16])]
    #[rauk]
    fn button_click(mut cx: button_click::Context) {
        // Clear interrupt
        cx.resources
            .button
            .lock(|button| button.clear_interrupt_pending_bit());

        // Some nonsensical work here...
        cx.resources.shared_u8.lock(|i| *i += 1);
        asm::delay(5_000);
        cx.resources.shared_u16.lock(|i| *i += 3);
    }

    // Periodic task that toggles the LED every 1s
    #[task(binds = TIM2, resources = [led, led_on, timer, shared_u8, shared_u16])]
    #[rauk]
    fn toggle_led(mut cx: toggle_led::Context) {
        // Clear interrupt
        cx.resources.timer.lock(|timer| {
            timer.clear_interrupt(Event::TimeOut);
        });

        let mut su8 = cx.resources.shared_u8;
        let mut su16 = cx.resources.shared_u16;

        // Check the shared resources and do some work here in rare cases
        su8.lock(|su8| {
            su16.lock(|su16| {
                if *su8 == 123 {
                    asm::delay(1_000);
                    if *su16 == 12345 {
                        asm::delay(10_000);
                        *su16 += 10;
                    }
                }
            });
        });

        let powered_on = cx.resources.led_on.lock(|led_on| *led_on);
        if !powered_on {
            cx.resources.led.lock(|led| {
                led.set_high().unwrap();
            });
            cx.resources.led_on.lock(|led_on| *led_on = true);
        } else {
            cx.resources.led.lock(|led| {
                led.set_low().unwrap();
            });
            cx.resources.led_on.lock(|led_on| *led_on = false);
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }
}
