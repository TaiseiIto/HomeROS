pub mod system;
mod table;

/// # References
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Char16 = u16;

/// # References
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Handle = *const Void;

/// # References
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Void = ();
