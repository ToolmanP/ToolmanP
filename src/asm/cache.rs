mod sealed {
    pub trait Dc {
        fn dc(&self);
    }

    pub trait Ic {
        fn ic(&self);
    }
}

macro_rules! dcache {
    ($A: ident) => {
        impl sealed::Dc for $A {
            fn dc(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {},

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    };
}

pub struct Isw;

impl sealed::Dc for Isw {
    #[inline(always)]
    fn dc(&self) {
        match () {
            #[cfg(target_arch = "aarch64")]
            () => unsafe {

            },

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    }
}
