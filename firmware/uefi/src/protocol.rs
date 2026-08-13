pub mod text;

/// # References
/// * [EFI_DEVICE_PATH_PROTOCOL](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-device-path-protocol)
#[repr(C)]
pub struct DevicePath {
    device_type: u8,
    sub_type: u8,
    length: [u8; 2],
}
