// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Barrier functions.

mod sealed {
    pub trait Dmb {
        fn dmb(&self);
    }

    pub trait Dsb {
        fn dsb(&self);
    }

    pub trait Isb {
        fn isb(&self);
    }
}

macro_rules! dmb_dsb {
    ($A:ident) => {
        pub struct $A;

        impl sealed::Dmb for $A {
            #[inline(always)]
            fn dmb(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("DMB ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
        impl sealed::Dsb for $A {
            #[inline(always)]
            fn dsb(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("DSB ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    };
}

dmb_dsb!(Sy);
dmb_dsb!(St);
dmb_dsb!(Ld);
dmb_dsb!(Ish);
dmb_dsb!(Ishst);
dmb_dsb!(Ishld);
dmb_dsb!(Nsh);
dmb_dsb!(Nshst);
dmb_dsb!(Nshld);
dmb_dsb!(Osh);
dmb_dsb!(Oshlt);
dmb_dsb!(Oshld);

impl sealed::Isb for Sy {
    #[inline(always)]
    fn isb(&self) {
        match () {
            #[cfg(target_arch = "aarch64")]
            () => unsafe { core::arch::asm!("ISB SY", options(nostack)) },

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    }
}

pub const SY: Sy = Sy {};
pub const ST: St = St {};
pub const LD: Ld = Ld {};
pub const ISH: Ish = Ish {};
pub const ISHLD: Ishld = Ishld {};
pub const NSH: Nshld = Nshld {};
pub const OSH: Osh = Osh {};
pub const OSHLT: Oshlt = Oshlt {};
pub const OSHLD: Oshld = Oshld {};

pub fn isb<T>(_arg: T)
where
    T: sealed::Isb,
{
    _arg.isb();
}

pub fn dsb<T>(_arg: T)
where
    T: sealed::Dsb,
{
    _arg.dsb();
}

pub fn dmb<T>(_arg: T)
where
    T: sealed::Dmb,
{
    _arg.dmb();
}
