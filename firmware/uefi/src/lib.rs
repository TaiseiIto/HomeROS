#![feature(string_into_chars)]
#![no_std]

extern crate alloc;

mod configuration;
mod protocol;
pub mod service;
pub mod system;
mod table;

use {
    alloc::{
        collections::btree_map::BTreeMap,
        format,
        string::{String, ToString},
        vec::Vec,
    },
    core::{
        fmt::{self, Debug, Formatter},
        str::FromStr,
    },
    regex::{Automaton, Capture, Match},
};

/// # References
/// * [EFI_GUID](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
#[derive(Eq, PartialEq)]
#[repr(C)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Debug for Guid {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[..2]
                .iter()
                .fold(0u16, |data, byte| (data << u8::BITS) + (*byte as u16)),
            self.data4[2..]
                .iter()
                .fold(0u64, |data, byte| (data << u8::BITS) + (*byte as u64)),
        ))
    }
}

impl FromStr for Guid {
    type Err = ();

    fn from_str(guid: &str) -> Result<Self, Self::Err> {
        let automaton: Automaton = r"^(?<data1>[\da-f]{8})-(?<data2>[\da-f]{4})-(?<data3>[\da-f]{4})-(?<data4prefix>[\da-f]{4})-(?<data4suffix>[\da-f]{12})$".parse().unwrap();
        let matches: Vec<Match> = automaton.input(guid);
        let mat: Match = matches.into_iter().next().ok_or(())?;
        let captures: BTreeMap<String, Vec<Capture>> = mat.captures();
        let data1: u32 =
            u32::from_str_radix(&captures["data1"].iter().next().unwrap().to_string(), 16)
                .map_err(|_| ())?;
        let data2: u16 =
            u16::from_str_radix(&captures["data2"].iter().next().unwrap().to_string(), 16)
                .map_err(|_| ())?;
        let data3: u16 =
            u16::from_str_radix(&captures["data3"].iter().next().unwrap().to_string(), 16)
                .map_err(|_| ())?;
        let data4prefix: Vec<char> = captures["data4prefix"]
            .iter()
            .next()
            .unwrap()
            .to_string()
            .into_chars()
            .collect();
        let data4prefix: Vec<u8> = data4prefix
            .chunks(2)
            .map(|byte| u8::from_str_radix(&format!("{}{}", byte[0], byte[1]), 16).unwrap())
            .collect();
        let data4prefix: [u8; 2] = data4prefix.try_into().map_err(|_| ())?;
        let data4suffix: Vec<char> = captures["data4suffix"]
            .iter()
            .next()
            .unwrap()
            .to_string()
            .into_chars()
            .collect();
        let data4suffix: Vec<u8> = data4suffix
            .chunks(2)
            .map(|byte| u8::from_str_radix(&format!("{}{}", byte[0], byte[1]), 16).unwrap())
            .collect();
        let data4suffix: [u8; 6] = data4suffix.try_into().map_err(|_| ())?;
        let data4: Vec<u8> = data4prefix
            .into_iter()
            .chain(data4suffix.into_iter())
            .collect();
        let data4: [u8; 8] = data4.try_into().map_err(|_| ())?;
        Ok(Self {
            data1,
            data2,
            data3,
            data4,
        })
    }
}

/// # References
/// * [EFI_STATUS](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
#[derive(Debug, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub struct Status(usize);

impl Status {
    /// # References
    /// * [Status Codes](https://uefi.org/specs/UEFI/2.11/Apx_D_Status_Codes.html)
    const SUCCESS: Self = Self(0);
    const BUFFER_TOO_SMALL: Self = Self((1 << (usize::BITS - 1)) + 5);

    pub fn assert(self) {
        assert_eq!(self, Self::SUCCESS);
    }

    pub fn assert_buffer_too_small(self) {
        assert_eq!(self, Self::BUFFER_TOO_SMALL);
    }
}

/// # References
/// * [CHAR16](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Char16 = u16;

/// # References
/// * [EFI_EVENT](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
/// * [EFI_EVENT](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createevent)
pub type Event = *const Void;

/// # References
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
pub type Handle = *const Void;

/// # References
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
pub type HandleMut = *mut Void;

/// # References
/// * [VOID](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Void = ();
