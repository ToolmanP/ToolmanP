mod sealed {
    pub trait Tlbi {
        fn invalidate(&self);
    }
}

macro_rules! tlbi {
    ($A: ident) => {
        pub struct $A;
        impl sealed::Tlbi for $A {
            #[inline(always)]
            fn invalidate(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("tlbi ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    };
}

tlbi!(Vmalle1is);
tlbi!(Vmalle2is);
tlbi!(Vmalle3is);

pub const VMALLE1IS: Vmalle1is = Vmalle1is {};
pub const VMALLE2IS: Vmalle2is = Vmalle2is {};
pub const VMALLE3IS: Vmalle3is = Vmalle3is {};

pub fn tlbi<T>(_arg: T)
where
    T: sealed::Tlbi,
{
    _arg.invalidate()
}
