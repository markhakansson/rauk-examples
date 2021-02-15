#![no_main]
#![no_std]

#[cfg(not(feature = "klee-analysis"))]
use panic_halt as _;
#[cfg(feature = "klee-analysis")]
use panic_klee as _;

#[doc = r" Implementation details"]
pub mod app {
    #[doc = r" Always include the device crate which contains the vector table"]
    use stm32f4xx_hal::stm32 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    #[doc = r" User code from within the module"]
    #[doc = r" User code end"]
    #[allow(non_snake_case)]
    fn init(cx: init::Context) -> init::LateResources {
        let start: u32 = cx.core.DWT.cyccnt.read();
        let end = cx.core.DWT.cyccnt.read();
        let _time = end - start;
        init::LateResources {}
    }
    #[allow(non_snake_case)]
    fn idle(cx: idle::Context) -> ! {
        use rtic::mutex_prelude::*;
        use rtic::Mutex as _;
        panic!("idle");
        loop {}
    }
    #[doc = r" Resources initialized at runtime"]
    #[allow(non_snake_case)]
    pub struct initLateResources {}
    #[allow(non_snake_case)]
    #[doc = "Initialization function"]
    pub mod init {
        #[doc(inline)]
        pub use super::initLateResources as LateResources;
        #[doc = r" Execution context"]
        pub struct Context<'a> {
            #[doc = r" Core (Cortex-M) peripherals"]
            pub core: rtic::export::Peripherals,
            #[doc = r" Device peripherals"]
            pub device: stm32f4xx_hal::stm32::Peripherals,
            #[doc = r" Critical section token for init"]
            pub cs: rtic::export::CriticalSection<'a>,
        }
        impl<'a> Context<'a> {
            #[inline(always)]
            pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
                Context {
                    device: stm32f4xx_hal::stm32::Peripherals::steal(),
                    cs: rtic::export::CriticalSection::new(),
                    core,
                }
            }
        }
    }
    #[allow(non_snake_case)]
    #[doc = "Idle loop"]
    pub mod idle {
        #[doc = r" Execution context"]
        pub struct Context {}
        impl Context {
            #[inline(always)]
            pub unsafe fn new(priority: &rtic::export::Priority) -> Self {
                Context {}
            }
        }
    }
    #[doc = r" Unused"]
    #[allow(non_camel_case_types)]
    pub enum Tasks {
        idle,
    }
    #[cfg(not(feature = "klee-analysis"))]
    #[doc = r" app module"]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            let _TODO: () = ();
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
            let late = crate::app::init(init::Context::new(core.into()));
            rtic::export::interrupt::enable();
            crate::app::idle(idle::Context::new(&rtic::export::Priority::new(0)))
        }
    }
    #[cfg(feature = "klee-analysis")]
    #[doc = r" klee module"]
    mod rtic_ext {
        use super::*;
        use klee_sys::klee_make_symbolic;
        #[no_mangle]
        unsafe extern "C" fn main() {
            let mut task = 0;
            klee_make_symbolic!(&mut task, "task");
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
            match task {
                0u32 => {
                    let late = crate::app::init(init::Context::new(core.into()));
                }
                1u32 => crate::app::idle(idle::Context::new(&rtic::export::Priority::new(0))),
                _ => {}
            }
        }
    }
}
