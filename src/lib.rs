#![no_std]
#![feature(const_trait_impl)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::doc_markdown)]

use core::{fmt::Display, num::NonZeroU32, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SoC {
    MT6572 = 0x6572,
    MT6575 = 0x6575,
    MT6577 = 0x6577,
    MT6582 = 0x6582,
    MT6595 = 0x6595,
    MT6739 = 0x699,
    MT6761 = 0x717,
    MT6765 = 0x766,
    MT6768 = 0x707,
    MT6853 = 0x996,
    MT6877 = 0x959,
    MT6885 = 0x816,
    MT6789 = 0x1208,
    MT6855 = 0x1129,
    MT6886 = 0x1229,
    MT6878 = 0x1375,
    MT6895 = 0x1172,
    MT6899 = 0x6899,
    MT6983 = 0x907,
    MT6991 = 0x1357,
    MT6993 = 0x1471,
    MT6858 = 0x1585,
    MT8696 = 0x908,
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
            Self::MT6575 => "MT6575",
            Self::MT6577 => "MT6577",
            Self::MT6582 => "MT6582",
            Self::MT6595 => "MT6595/MT6595M/MT6595T",
            Self::MT6739 => "MT6739/MT6731/MT8765",
            Self::MT6761 => {
                "MT6761/MT6761V/WE/MT6761V/WAB/MT6761V/WBB/MT6762/MT6762G/MT6762V/WB/MT6762V/WD/MT3369/MT8766B/MT8761/AC8259/AC8257"
            }
            Self::MT6765 => {
                "MT6765/MT6765G/MT6765H/MT6765V/MT6765V/CB/MT6765V/XAA/MT6765V/XBA/MT8768T"
            }
            Self::MT6768 => {
                "MT6768/MT6769/MT6769V/CB/MT6769T/MT6769V/CT/MT6769V/CU/MT6769J/MT6769L/MT6769S/MT6769Z/MT6769V/CZ/MT6769H/MT6769G/MT6769K/MT6769I"
            }
            Self::MT6853 => "MT6853/MT6853T/MT6853V/NZA/MT6853V/TNZA",
            Self::MT6877 => {
                "MT6877/MT6877T/MT6877V/ZA/MT6877V/TZA/MT6877V/TTZA/MT6877V_T/TTZA/MT8791/MT8791N"
            }
            Self::MT6885 => {
                "MT6885/MT6885Z/CZA/MT6883/MT6883Z/CZA/MT6889/MT6889Z/CZA/MT6880/MT6890"
            }
            Self::MT6789 => {
                "MT6789/MT6789G/MT6789U/MT6789V/CD/MT6789H/MT6789I/MT6789J/MT6789T/MT8781/MT8781V/CA/MT8781V/NA"
            }
            Self::MT6855 => "MT6855/MT6855G/MT6855V/AZA/MT6855V/ATZA/MT6855V_A/ATZA/MT6855V/TTZA",
            Self::MT6886 => "MT6886/MT6886V_A/CZA/MT6886V_B/CZA/MT6886V/TCZA",
            Self::MT6878 => {
                "MT6878/MT6878V/ZA/MT6878V_A/ZA/MT6878V_B/ZA/MT6878V_E/ZA/MT6878V/FZA/MT6878V_G/ZA/MT6878V/TZA/MT6878V/TFZA"
            }
            Self::MT6895 => {
                "MT6895/MT6895Z/CZA/MT6895Z/TCZA/MT6895Z_A/TCZA/MT6895Z_B/TCZA/MT6895ZB/MT8795/MT8795Z/TNZA/MT6896/MT6896Z/CZA/MT6896Z_B/CZA/MT6896Z_C/CZA"
            }
            Self::MT6899 => {
                "MT6899/MT6899Z/ZA/MT6899Z_A/ZA/MT6899Z_B/ZA/MT6899Z_C/ZA/MT6899Z_E/ZA/MT6899Z_D/ZA/MT6899Z_A/TZA/MT6899Z_T/TZA"
            }
            Self::MT6983 => "MT6983/MT6983Z/CZA/MT6983W/CZA/MT8798/MT8798Z/CNZA/MT8798Z/TNZA",
            Self::MT6991 => {
                "MT6991/MT6991Z/CZA/MT6991W/CZA/MT6991Z/TCZA/MT6991Z/TCZB/MT6991Z/ECZB/MT8799Z/TNZB"
            }
            Self::MT6993 => "MT6993/MT6993W/CZA",
            Self::MT6858 => "MT6858/MT6858V/ZA/MT6858T",
            Self::MT8696 => "MT8696",
        }
    }

    /// get SoC marketing name
    #[must_use]
    pub const fn marketing_name(self) -> Option<&'static str> {
        match self {
            Self::MT6572
            | Self::MT6575
            | Self::MT6577
            | Self::MT6582
            | Self::MT6595
            | Self::MT6739
            | Self::MT8696 => None,
            Self::MT6761 => Some("Helio A20/A22/A25/P22/G25"),
            Self::MT6765 => Some("Helio P35/G35/G36/G37/G50"),
            Self::MT6768 => Some(
                "Helio P65/G70/G80/G81/G81 Ultra/G81 Extreme/G85/G88/G91/G91 Ultra/G92/G92 Max",
            ),
            Self::MT6853 => Some("Dimensity 720/800U"),
            Self::MT6877 => Some("Dimensity 900/920/1080/7050"),
            Self::MT6885 => Some("Dimensity 1000C/1000L/1000/1000+"),
            Self::MT6789 => Some("Helio G99/G100/G200"),
            Self::MT6855 => Some("Dimensity 930/7020/7025/7060"),
            Self::MT6886 => Some("Dimensity 7200/7350"),
            Self::MT6878 => Some("Dimensity 7300/7300X/7360/7400/7400X"),
            Self::MT6895 => Some("Dimensity 8000/8100/8200/8250"),
            Self::MT6899 => Some("Dimensity 8400/8400-Ultra/8400-Turbo/8450/8500/8550"),
            Self::MT6983 => Some("Dimensity 9000/9000+"),
            Self::MT6991 => Some("Dimensity 9400/9400+/9500s"),
            Self::MT6993 => Some("Dimensity 9500"),
            Self::MT6858 => Some("Dimensity 7100/7300e"),
        }
    }
}

/// SoC MMIO
pub const trait MMIO: Sized {
    /// get BootROM base address
    fn bootrom(self) -> u32;
    /// get devinfo (region with hwcode/subcode/...) MMIO address
    fn devinfo(self) -> u32;
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
    /// get SSR (Scalable Security Root) MMIO address
    fn ssr(self) -> Option<NonZeroU32>;

    /// get SoC from the hwcode
    fn try_from_hwcode(hwcode: u16) -> Option<Self>;
    /// get hwcode for the SoC
    fn to_hwcode(self) -> u16;

    /// get SoC from the dacode
    fn try_from_dacode(dacode: u16) -> Option<Self>;
    /// get dacode for the SoC
    fn to_dacode(self) -> u16;
}

impl MMIO for SoC {
    fn bootrom(self) -> u32 {
        match self {
            Self::MT6572 | Self::MT6582 => 0x00400000,
            Self::MT6575 | Self::MT6577 => 0xffff0000,
            Self::MT6595
            | Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT6855
            | Self::MT6886
            | Self::MT6878
            | Self::MT6895
            | Self::MT6899
            | Self::MT6983
            | Self::MT6991
            | Self::MT6993
            | Self::MT6858
            | Self::MT8696 => 0x00000000,
        }
    }

    fn devinfo(self) -> u32 {
        match self {
            Self::MT6991 | Self::MT6993 => 0x00f00000,
            _ => 0x08000000, // XXX: not confirmed for 6577
        }
    }

    fn tzcc(self) -> Option<NonZeroU32> {
        match self {
            Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT6855
            | Self::MT6895
            | Self::MT6983 => Some(nz(0x10210000)),
            Self::MT6886 => Some(nz(0x1c807000)),
            _ => None,
        }
    }

    fn toprgu(self) -> u32 {
        match self {
            Self::MT6575 | Self::MT6577 => 0xc0000000, // XXX: not confirmed
            Self::MT6572
            | Self::MT6582
            | Self::MT6595
            | Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT8696 => 0x10007000,
            Self::MT6855 | Self::MT6886 | Self::MT6895 | Self::MT6983 => 0x1c007000,
            Self::MT6878 | Self::MT6858 => 0x1c00a000,
            Self::MT6899 => 0x1c00b000,
            Self::MT6991 | Self::MT6993 => 0x1c010000,
        }
    }

    fn apxgpt(self) -> Option<NonZeroU32> {
        match self {
            Self::MT6575 | Self::MT6577 => Some(nz(0xc1002000)), // XXX: not confirmed
            Self::MT6572
            | Self::MT6582
            | Self::MT6595
            | Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT8696 => Some(nz(0x10008000)),
            Self::MT6855 | Self::MT6886 | Self::MT6878 | Self::MT6895 | Self::MT6983 => {
                Some(nz(0x1c008000))
            }
            Self::MT6899 | Self::MT6991 | Self::MT6993 | Self::MT6858 => None, // XXX: not found
        }
    }

    fn efuse(self) -> u32 {
        match self {
            Self::MT6572 => 0x10009000,
            Self::MT6575 | Self::MT6577 => 0xc1019000, // XXX: not confirmed
            Self::MT6582 => todo!(),
            Self::MT6595 => 0x10206000,
            Self::MT6739 => 0x11c00000,
            Self::MT6761 | Self::MT6765 => 0x11C50000,
            Self::MT6768 => 0x11ce0000,
            Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT6878
            | Self::MT6895
            | Self::MT6899
            | Self::MT6983 => 0x11f10000,
            Self::MT6855 | Self::MT8696 => 0x11c10000,
            Self::MT6886 => 0x11e30000,
            Self::MT6991 => 0x13260000,
            Self::MT6993 => 0x10160000,
            Self::MT6858 => 0x11ea0000,
        }
    }

    fn hacc(self) -> u32 {
        match self {
            Self::MT6575 | Self::MT6577 => 0xc101a000, // XXX: not confirmed
            Self::MT6572
            | Self::MT6582
            | Self::MT6595
            | Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT8696 => 0x1000a000,
            Self::MT6855 | Self::MT6886 | Self::MT6895 | Self::MT6983 => 0x1c009000,
            Self::MT6878 | Self::MT6899 | Self::MT6858 => 0x1040e000,
            Self::MT6991 | Self::MT6993 => 0x1800e000,
        }
    }

    fn uart0(self) -> u32 {
        match self {
            Self::MT6572 => 0x11005000,
            Self::MT6575 | Self::MT6577 => 0xffffff00, // XXX: not confirmed
            Self::MT6582
            | Self::MT6595
            | Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789 => 0x11002000,
            Self::MT6855
            | Self::MT6886
            | Self::MT6878
            | Self::MT6895
            | Self::MT6899
            | Self::MT6983
            | Self::MT6858 => 0x11001000,
            Self::MT6991 | Self::MT6993 => 0x16000000,
            Self::MT8696 => 0x11002400,
        }
    }

    fn ssr(self) -> Option<NonZeroU32> {
        match self {
            Self::MT6878 | Self::MT6899 | Self::MT6858 => Some(nz(0x10400000)),
            Self::MT6991 | Self::MT6993 => Some(nz(0x18000000)),
            _ => None,
        }
    }

    fn try_from_hwcode(hwcode: u16) -> Option<Self> {
        match hwcode {
            0x6572 => Some(Self::MT6572),
            0x6575 => Some(Self::MT6575),
            0x6577 => Some(Self::MT6577),
            0x6582 => Some(Self::MT6582),
            0x6595 => Some(Self::MT6595),
            0x699 => Some(Self::MT6739),
            0x717 => Some(Self::MT6761),
            0x766 => Some(Self::MT6765),
            0x707 => Some(Self::MT6768),
            0x996 => Some(Self::MT6853),
            0x959 => Some(Self::MT6877),
            0x816 => Some(Self::MT6885),
            0x1208 => Some(Self::MT6789),
            0x1129 => Some(Self::MT6855),
            0x1229 => Some(Self::MT6886),
            0x1375 => Some(Self::MT6878),
            0x1172 => Some(Self::MT6895),
            0x6899 => Some(Self::MT6899),
            0x907 => Some(Self::MT6983),
            0x1357 => Some(Self::MT6991),
            0x1471 => Some(Self::MT6993),
            0x908 => Some(Self::MT8696),
            0x1585 => Some(Self::MT6858),
            _ => None,
        }
    }

    fn to_hwcode(self) -> u16 {
        self as u16
    }

    fn try_from_dacode(dacode: u16) -> Option<Self> {
        match dacode {
            0x6572 => Some(Self::MT6572),
            0x6575 => Some(Self::MT6575),
            0x6577 => Some(Self::MT6577),
            0x6582 => Some(Self::MT6582),
            0x6595 => Some(Self::MT6595),
            0x6739 => Some(Self::MT6739),
            0x6761 => Some(Self::MT6761),
            0x6765 => Some(Self::MT6765),
            0x6768 => Some(Self::MT6768),
            0x6853 => Some(Self::MT6853),
            0x6877 => Some(Self::MT6877),
            0x6885 => Some(Self::MT6885),
            0x1208 => Some(Self::MT6789),
            0x1129 => Some(Self::MT6855),
            0x1229 => Some(Self::MT6886),
            0x1375 => Some(Self::MT6878),
            0x1172 => Some(Self::MT6895),
            0x6899 => Some(Self::MT6899),
            0x907 => Some(Self::MT6983),
            0x1357 => Some(Self::MT6991),
            0x1471 => Some(Self::MT6993),
            0x1585 => Some(Self::MT6858),
            0x8696 => Some(Self::MT8696),
            _ => None,
        }
    }

    fn to_dacode(self) -> u16 {
        match self {
            Self::MT6572
            | Self::MT6575
            | Self::MT6577
            | Self::MT6582
            | Self::MT6595
            | Self::MT6789
            | Self::MT6855
            | Self::MT6886
            | Self::MT6878
            | Self::MT6895
            | Self::MT6899
            | Self::MT6983
            | Self::MT6991
            | Self::MT6993
            | Self::MT6858 => self.to_hwcode(),
            Self::MT6739 => 0x6739,
            Self::MT6761 => 0x6761,
            Self::MT6765 => 0x6765,
            Self::MT6768 => 0x6768,
            Self::MT6853 => 0x6853,
            Self::MT6877 => 0x6877,
            Self::MT6885 => 0x6885,
            Self::MT8696 => 0x8696,
        }
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
            Self::MT6595 => 0x2000000..0x2040000,
            _ => todo!(),
        }
    }

    fn dram_start(self) -> u32 {
        match self {
            Self::MT6572 | Self::MT6582 => 0x80000000,
            Self::MT6575 | Self::MT6577 => 0x00000000,
            Self::MT6595
            | Self::MT6739
            | Self::MT6761
            | Self::MT6765
            | Self::MT6768
            | Self::MT6853
            | Self::MT6877
            | Self::MT6885
            | Self::MT6789
            | Self::MT6855
            | Self::MT6886
            | Self::MT6878
            | Self::MT6895
            | Self::MT6899
            | Self::MT6983
            | Self::MT6858
            | Self::MT8696 => 0x40000000,
            Self::MT6991 | Self::MT6993 => 0x80000000,
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
