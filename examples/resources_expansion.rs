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
    fn init(_cx: init::Context) -> init::LateResources {
        let value = 30;
        init::LateResources { a: value }
    }
    #[allow(non_snake_case)]
    fn idle(_cx: idle::Context) -> ! {
        use rtic::mutex_prelude::*;
        use rtic::Mutex as _;
        panic!("idle");
    }
    #[allow(non_snake_case)]
    fn increment(mut cx: increment::Context) {
        use rtic::mutex_prelude::*;
        use rtic::Mutex as _;
        cx.resources.a.lock(|a| *a + 1);
    }
    #[allow(non_snake_case)]
    fn decrement(mut cx: decrement::Context) {
        use rtic::mutex_prelude::*;
        use rtic::Mutex as _;
        cx.resources.a.lock(|a| *a - 1);
        cx.resources.b.lock(|b| *b - 10);
    }
    #[doc = r" Resources initialized at runtime"]
    #[allow(non_snake_case)]
    pub struct initLateResources {
        pub a: u8,
    }
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
    mod resources {
        use rtic::export::Priority;
        #[allow(non_camel_case_types)]
        pub struct a<'a> {
            priority: &'a Priority,
        }
        impl<'a> a<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                a { priority }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
        #[allow(non_camel_case_types)]
        pub struct b<'a> {
            priority: &'a Priority,
        }
        impl<'a> b<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                b { priority }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
    }
    #[allow(non_snake_case)]
    #[doc = "Resources `increment` has access to"]
    pub struct incrementResources<'a> {
        pub a: resources::a<'a>,
    }
    #[allow(non_snake_case)]
    #[doc = "Hardware task"]
    pub mod increment {
        #[doc(inline)]
        pub use super::incrementResources as Resources;
        #[doc = r" Execution context"]
        pub struct Context<'a> {
            #[doc = r" Resources this task has access to"]
            pub resources: Resources<'a>,
        }
        impl<'a> Context<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
                Context {
                    resources: Resources::new(priority),
                }
            }
        }
    }
    #[allow(non_snake_case)]
    #[doc = "Resources `decrement` has access to"]
    pub struct decrementResources<'a> {
        pub a: resources::a<'a>,
        pub b: resources::b<'a>,
    }
    #[allow(non_snake_case)]
    #[doc = "Hardware task"]
    pub mod decrement {
        #[doc(inline)]
        pub use super::decrementResources as Resources;
        #[doc = r" Execution context"]
        pub struct Context<'a> {
            #[doc = r" Resources this task has access to"]
            pub resources: Resources<'a>,
        }
        impl<'a> Context<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
                Context {
                    resources: Resources::new(priority),
                }
            }
        }
    }
    #[doc = r" Unused"]
    #[allow(non_camel_case_types)]
    pub enum Tasks {
        idle,
        increment,
        decrement,
    }
    #[doc = r" app module"]
    #[allow(non_upper_case_globals)]
    #[link_section = ".uninit.rtic0"]
    static mut __rtic_internal_a: core::mem::MaybeUninit<u8> = core::mem::MaybeUninit::uninit();
    impl<'a> rtic::Mutex for resources::a<'a> {
        type T = u8;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u8) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            #[doc = r" Priority ceiling"]
            const CEILING: u8 = 1u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_a.as_mut_ptr(),
                    self.priority(),
                    CEILING,
                    stm32f4xx_hal::stm32::NVIC_PRIO_BITS,
                    f,
                )
            }
        }
    }
    #[allow(non_upper_case_globals)]
    static mut __rtic_internal_b: u16 = 10;
    impl<'a> rtic::Mutex for resources::b<'a> {
        type T = u16;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u16) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            #[doc = r" Priority ceiling"]
            const CEILING: u8 = 1u8;
            unsafe {
                rtic::export::lock(
                    &mut __rtic_internal_b,
                    self.priority(),
                    CEILING,
                    stm32f4xx_hal::stm32::NVIC_PRIO_BITS,
                    f,
                )
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn EXTI0() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || {
            crate::app::increment(increment::Context::new(&rtic::export::Priority::new(
                PRIORITY,
            )))
        });
    }
    impl<'a> incrementResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            incrementResources {
                a: resources::a::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn EXTI1() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(PRIORITY, || {
            crate::app::decrement(decrement::Context::new(&rtic::export::Priority::new(
                PRIORITY,
            )))
        });
    }
    impl<'a> decrementResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            decrementResources {
                a: resources::a::new(priority),
                b: resources::b::new(priority),
            }
        }
    }
    #[cfg(not(feature = "klee-analysis"))]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            let _TODO: () = ();
            rtic::export::assert_send::<u8>();
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
            let _ = [(); ((1 << stm32f4xx_hal::stm32::NVIC_PRIO_BITS) - 1u8 as usize)];
            core.NVIC.set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::EXTI0,
                rtic::export::logical2hw(1u8, stm32f4xx_hal::stm32::NVIC_PRIO_BITS),
            );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::EXTI0,
            );
            let _ = [(); ((1 << stm32f4xx_hal::stm32::NVIC_PRIO_BITS) - 1u8 as usize)];
            core.NVIC.set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::EXTI1,
                rtic::export::logical2hw(1u8, stm32f4xx_hal::stm32::NVIC_PRIO_BITS),
            );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::EXTI1,
            );
            let late = crate::app::init(init::Context::new(core.into()));
            __rtic_internal_a.as_mut_ptr().write(late.a);
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
            let mut a: u8 = 0;
            let mut b: u16 = 0;
            klee_make_symbolic!(&mut task, "task");
            klee_make_symbolic!(&mut a, "a");
            klee_make_symbolic!(&mut b, "b");
            __rtic_internal_a.as_mut_ptr().write(a);
            __rtic_internal_b = b;
            match task {
                0u32 => {
                    crate::app::increment(increment::Context::new(&rtic::export::Priority::new(1)));
                }
                1u32 => {
                    crate::app::decrement(decrement::Context::new(&rtic::export::Priority::new(1)));
                }
                2u32 => {
                    let mut core: rtic::export::Peripherals =
                        rtic::export::Peripherals::steal().into();
                    crate::app::init(init::Context::new(core.into()));
                }
                3u32 => {
                    crate::app::idle(idle::Context::new(&rtic::export::Priority::new(0)));
                }
                _ => {}
            }
        }
    }
}
