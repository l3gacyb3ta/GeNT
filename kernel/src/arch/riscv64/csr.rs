bitfield::bitfield! {
    pub struct Satp(u64);
    impl Debug;

    ppn, set_ppn: 43, 0;
    asid, set_asid: 59, 44;
    mode_raw, set_mode_raw: 63, 60;
}

impl Satp {
    pub fn phys(&self) -> crate::mem::PhysicalAddress {
        crate::mem::PhysicalAddress::new((self.ppn() << 12) as usize)
    }

    pub fn mode(&self) -> super::paging::Mode {
        use super::paging::Mode;

        match self.mode_raw() {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            10 => Mode::Sv57,
            11 => Mode::Sv64,
            _ => panic!("Unrecognized paging mode")
        }
    }
}