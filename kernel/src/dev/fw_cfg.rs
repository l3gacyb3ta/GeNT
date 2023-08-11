use core::cell::OnceCell;

use alloc::vec::Vec;
use libsa::{volatile::Volatile, endian::{BigEndianU16, BigEndianU32, BigEndianU64}};

use crate::dma::DmaRange;

const DMA_OFFSET: usize = 16;
const SEL_OFFSET: usize = 8;

pub struct FwCfg {
    transit: crate::arch::transit::IOTransit,
    files: OnceCell<Vec<FwCfgFile>>,
}

impl FwCfg {
    pub fn new(location: usize) -> Self {
        Self { 
            transit: crate::arch::transit::IOTransit::new(location),
            files: OnceCell::new()
        }
    }

    /// Returns a list of all available files
    pub fn files(&self) -> &[FwCfgFile] {
        self.files.get_or_init(|| {
            self.transit.write(SEL_OFFSET, 0x19_u16.to_be());
            (0..u32::from_be(self.transit.read::<u32>(0)))
                .map(|_| unsafe {
                        self.transit.read_serial::<_, u8>(0)
                    }
                )
                .collect()
        })
    }

    /// Search for a file at the given `path`
    pub fn lookup(&self, path: &str) -> Option<&FwCfgFile> {
        self.files().iter().find(|f| f.name() == Some(path))
    }

    /// Write the contents of a buffer into a file
    pub fn write_file(&self, file: &FwCfgFile, buf: &[u8]) -> Result<(), Error> {
        let mut dma: DmaRange<[u8]> = DmaRange::new_many(file.size().get() as usize);

        for (index, dmacell) in dma.buf_mut().iter_mut().enumerate() {
            if index >= buf.len() {
                break;
            }

            *dmacell = buf[index];
        }

        unsafe {
            self.dma_command(
                Some(file.sel()),
                DmaCommand::SELECT | DmaCommand::WRITE,
                file.size().get(),
                dma.phys() as _,
            )?;

            Ok(())
        }
    }

    /// Read the contents of a file into a buffer
    pub fn read_file(&self, file: &FwCfgFile) -> Result<Vec<u8>, Error> {
        let dma: DmaRange<[u8]> = DmaRange::new_many(file.size().get() as usize);

        unsafe {
            self.dma_command(
                Some(file.sel()),
                DmaCommand::SELECT | DmaCommand::READ,
                file.size().get(),
                dma.phys() as _,
            )?;

            let mut res = Vec::new();

            for cell in dma.buf().iter() {
                res.push(*cell);
            }

            Ok(res)
        }
    }

    unsafe fn dma_command(
        &self,
        sel: Option<BigEndianU16>,
        cmd: DmaCommand,
        length: u32,
        address: u64,
    ) -> Result<(), Error> {
        let control = ((sel.unwrap_or_default().get() as u32) << 16) | cmd.bits();
        let packet = FwCfgPacket {
            control: Volatile::new(BigEndianU32::new(control)),
            length: BigEndianU32::new(length),
            address: BigEndianU64::new(address),
        };

        let mut dma = crate::dma::DmaRange::new();

        *dma = packet;

        // Issue the command by writing the address of the DmaPacket to
        // the DMA Control Register.
        self.transit.write(
            DMA_OFFSET, 
            (dma.phys() as u64).to_be()
        );

        // Wait for completion or error
        //
        // Currently QEMU completes all commands immediately, so we likely won't wait
        // at all, but fw_cfg may become asynchronous in the future making this necessary.
        loop {
            let ctrl = dma.control.read().get();

            if ctrl & 0x1 != 0 {
                return Err(Error::DmaError);
            }
            if ctrl == 0 {
                break;
            }
        }

        Ok(())
    }
}

#[repr(C)]
struct FwCfgPacket {
    pub control: Volatile<BigEndianU32>,
    pub length: BigEndianU32,
    pub address: BigEndianU64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    BadPointer,
    DmaError,
}

bitflags::bitflags! {
    #[repr(transparent)]
    struct DmaCommand : u32 {
        const ERROR     = 1 << 0;
        const READ      = 1 << 1;
        const SKIP      = 1 << 2;
        const SELECT    = 1 << 3;
        const WRITE     = 1 << 4;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FwCfgFile {
    size: BigEndianU32,
    select: BigEndianU16,
    _res: BigEndianU16,
    name: [u8; 56]
}

impl FwCfgFile {
    pub const fn size(&self) -> BigEndianU32 {
        self.size
    }

    pub const fn sel(&self) -> BigEndianU16 {
        self.select
    }

    pub fn name(&self) -> Option<&str> {
        let mut len = 0;
        while len < self.name.len() {
            if self.name[len] == 0 {
                break;
            }
            len += 1;
        }
        core::str::from_utf8(&self.name[..len]).ok()
    }
}

impl core::fmt::Debug for FwCfgFile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for character in self.name.iter() {
            write!(f, "{}", *character as char)?;
        }

        Ok(())
    }
}