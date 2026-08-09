use {super::Time, crate::Status};

/// # References
/// * [EFI_TIME_CAPABILITIES](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#gettime)
#[repr(C)]
pub struct Capabilities {
    resolution: u32,
    accuracy: u32,
    set_to_zero: bool,
}

/// # References
/// * [GetTime](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#gettime)
pub type Get = extern "efiapi" fn(*mut Time, *mut Capabilities) -> Status;
