use tock_registers::register_bitfields;

register_bitfields! {u64,
    VAddr [
        L0     OFFSET(39) NUMBITS(9),
        L1     OFFSET(30) NUMBITS(9),
        L2     OFFSET(21) NUMBITS(9),
        L3     OFFSET(12) NUMBITS(9),
        OFFSET OFFSET(0) NUMBITS(12)
    ]
}

#[repr(transparent)]
#[derive(Clone)]
pub struct VirtAddress(pub u64);

#[repr(transparent)]
#[derive(Clone)]
pub struct PhyAddress(pub u64);

#[repr(C)]
pub struct VirtLayout {
    pub indexes: [u64; 4],
    pub offset: u64,
}

impl VirtAddress {
    pub fn layout(&self) -> VirtLayout {
        VirtLayout {
            indexes: [
                VAddr::L0.read(self.0),
                VAddr::L1.read(self.0),
                VAddr::L2.read(self.0),
                VAddr::L3.read(self.0),
            ],
            offset: VAddr::OFFSET.read(self.0),
        }
    }
}

impl From<u64> for VirtAddress {
    fn from(value: u64) -> Self {
        VirtAddress(value)
    }
}

impl From<u64> for PhyAddress {
    fn from(value: u64) -> Self {
        PhyAddress(value)
    }
}
