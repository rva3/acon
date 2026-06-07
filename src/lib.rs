#![no_std]
#![feature(const_trait_impl)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::doc_markdown)]

use core::{fmt::Display, num::NonZeroU32, ops::Range, ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SoC {
    MT6572 = 0x6572,
    MT6577 = 0x6577,
    MT6595 = 0x6595,
    MT6768 = 0x707,
}

impl Display for SoC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MediaTek {}", self.segment_name())?;

        if let Some(marketing_name) = self.marketing_name() {
            write!(f, " ({marketing_name})")?;
        }

        Ok(())
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDisplay for SoC {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "MediaTek {}", self.segment_name())?;

        if let Some(marketing_name) = self.marketing_name() {
            ufmt::uwrite!(f, " ({})", marketing_name)?;
        }

        Ok(())
    }
}

impl SoC {
    /// get SoC segment name
    #[must_use]
    pub const fn segment_name(self) -> &'static str {
        match self {
            Self::MT6572 => "MT6572/MT6572M/MT6572A/MT6572W",
            Self::MT6577 => "MT6577",
            Self::MT6595 => "MT6595/MT6595M",
            Self::MT6768 => {
                "MT6768/MT6769/MT6769V/CB/MT6769T/MT6769V/CT/MT6769V/CU/MT6769J/MT6769L/MT6769S/MT6769Z/MT6769V/CZ/MT6769H/MT6769G/MT6769K/MT6769I"
            }
        }
    }

    /// get SoC marketing name
    #[must_use]
    pub const fn marketing_name(self) -> Option<&'static str> {
        match self {
            Self::MT6572 | Self::MT6577 | Self::MT6595 => None,
            Self::MT6768 => Some(
                "Helio P65/G70/G80/G81/G81 Ultra/G81 Extreme/G85/G88/G91/G91 Ultra/G92/G92 Max",
            ),
        }
    }
}

/// SoC MMIO
pub const trait MMIO: Sized {
    /// get BootROM base address
    fn bootrom(self) -> u32;
    /// get devinfo (region with hwcode/subcode/...) MMIO address
    fn devinfo() -> u32;
    /// get Trust Zone Crypto Cell MMIO address
    fn tzcc(self) -> Option<NonZeroU32>;
    /// get TOP Reset Generator Unit (watchdog) MMIO address
    fn toprgu(self) -> u32;
    /// get APXGPT MMIO address
    fn apxgpt(self) -> Option<NonZeroU32>;
    /// get EFUSE MMIO address
    fn efuse(self) -> u32;
    /// get HACC MMIO address
    fn hacc(self) -> u32;
    /// get UART0 MMIO address
    fn uart0(self) -> u32;

    /// get SoC from the hwcode
    fn try_from_hwcode(hwcode: u16) -> Option<Self>;
    /// get hwcode for the SoC
    fn to_hwcode(self) -> u16;

    /// get SoC from the dacode
    fn try_from_dacode(dacode: u16) -> Option<Self>;
    /// get dacode for the SoC
    fn to_dacode(self) -> u16;

    /// read value from the [`SoC::devinfo`] and try to determine the SoC
    ///
    /// # Safety
    /// safe only when running on the MediaTek chips
    unsafe fn try_from_mmio() -> Option<Self>;
}

impl MMIO for SoC {
    fn bootrom(self) -> u32 {
        match self {
            Self::MT6572 | Self::MT6595 | Self::MT6768 => 0x00400000,
            Self::MT6577 => 0xffff0000,
        }
    }

    fn devinfo() -> u32 {
        0x08000000 // XXX: not confirmed for 6577
    }

    fn tzcc(self) -> Option<NonZeroU32> {
        match self {
            Self::MT6768 => Some(nz(0x10210000)),
            _ => None,
        }
    }

    fn toprgu(self) -> u32 {
        match self {
            Self::MT6577 => 0xc0000000, // XXX: not confirmed
            Self::MT6572 | Self::MT6595 | Self::MT6768 => 0x10007000,
        }
    }

    fn apxgpt(self) -> Option<NonZeroU32> {
        match self {
            Self::MT6577 => Some(nz(0xc1002000)), // XXX: not confirmed
            Self::MT6572 | Self::MT6595 | Self::MT6768 => Some(nz(0x10008000)),
        }
    }

    fn efuse(self) -> u32 {
        match self {
            Self::MT6572 => 0x10009000,
            Self::MT6577 => 0xc1019000, // XXX: not confirmed
            Self::MT6595 => 0x10206000,
            Self::MT6768 => 0x11ce0000,
        }
    }

    fn hacc(self) -> u32 {
        match self {
            Self::MT6577 => 0xc101a000, // XXX: not confirmed
            Self::MT6572 | Self::MT6595 | Self::MT6768 => 0x1000a000,
        }
    }

    fn uart0(self) -> u32 {
        match self {
            Self::MT6572 => 0x11005000,
            Self::MT6577 => 0xc1009000, // XXX: not confirmed
            Self::MT6595 | Self::MT6768 => 0x11002000,
        }
    }

    fn try_from_hwcode(hwcode: u16) -> Option<Self> {
        match hwcode {
            0x6572 => Some(Self::MT6572),
            0x6577 => Some(Self::MT6577),
            0x6595 => Some(Self::MT6595),
            0x707 => Some(Self::MT6768),
            _ => None,
        }
    }

    fn to_hwcode(self) -> u16 {
        self as u16
    }

    fn try_from_dacode(dacode: u16) -> Option<Self> {
        match dacode {
            0x6572 => Some(Self::MT6572),
            0x6577 => Some(Self::MT6577),
            0x6595 => Some(Self::MT6595),
            0x6768 => Some(Self::MT6768),
            _ => None,
        }
    }

    fn to_dacode(self) -> u16 {
        match self {
            Self::MT6572 | Self::MT6577 | Self::MT6595 => self.to_hwcode(),
            Self::MT6768 => 0x6768,
        }
    }

    unsafe fn try_from_mmio() -> Option<Self> {
        let hwcode = unsafe { ptr::read_volatile(Self::devinfo() as _) };
        Self::try_from_hwcode(hwcode)
    }
}

/// SoC memory ranges
pub const trait Memory {
    /// get L2 cache usable range
    fn l2_sram(self) -> Range<u32>;
    /// get DRAM start address
    fn dram_start(self) -> u32;
}

impl Memory for SoC {
    fn l2_sram(self) -> Range<u32> {
        match self {
            // BUG: 0x2000000~0x2001000 is unusable
            Self::MT6572 => 0x2001000..0x2020000,
            _ => todo!(),
        }
    }

    fn dram_start(self) -> u32 {
        match self {
            Self::MT6572 => 0x80000000,
            Self::MT6577 => todo!(), // XXX: unk
            Self::MT6595 => todo!(), // XXX: unk
            Self::MT6768 => 0x40000000,
        }
    }
}

/// SoC BootROM function addresses
pub const trait BootROM: MMIO {
    fn usbdl_put_data(self) -> u32;
    fn usbdl_get_data(self) -> u32;
}

impl BootROM for SoC {
    fn usbdl_put_data(self) -> u32 {
        let base = self.bootrom();
        match self {
            Self::MT6572 => (base + 0xba4a) | 1,
            _ => todo!(),
        }
    }

    fn usbdl_get_data(self) -> u32 {
        let base = self.bootrom();
        match self {
            Self::MT6572 => (base + 0xb9c4) | 1,
            _ => todo!(),
        }
    }
}

const fn nz(v: u32) -> NonZeroU32 {
    NonZeroU32::new(v).expect("value is zero")
}
