pub mod multiple;

use crate::{Event, Guid, Status, Void, protocol::DevicePath};

/// References
/// * [EFI_LOCATE_SEARCH_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locatehandle)
#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

/// References
/// * [EFI_OPEN_PROTOCOL_INFORMATION_ENTRY](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-openprotocolinformation)
#[repr(C)]
pub struct OpenInformationEntry {
    agent: Handle,
    controller: Handle,
    attributes: u32,
    count: u32,
}

/// References
/// * [EFI_INTERFACE_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
#[repr(C)]
pub enum Type {
    Native,
}

/// References
/// * [CloseProtocol](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-closeprotocol)
#[must_use]
pub type Close = extern "efiapi" fn(Handle, *const Guid, Handle, Handle) -> Status;

/// References
/// * [HandleProtocol](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-handleprotocol)
#[must_use]
pub type Handle = extern "efiapi" fn(crate::Handle, *const Guid, *mut *const Void) -> Status;

/// References
/// * [InstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
#[must_use]
pub type Install =
    extern "efiapi" fn(*const crate::Handle, *const Guid, Type, *const Void) -> Status;

/// References
/// * [LocateProtocol](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locateprotocol)
#[must_use]
pub type Locate = extern "efiapi" fn(*const Guid, *const Void, *mut *const Void) -> Status;

/// References
/// * [LocalteDevicePath](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locatedevicepath)
#[must_use]
pub type LocateDevicePath =
    extern "efiapi" fn(*const Guid, *mut *const DevicePath, *mut crate::Handle) -> Status;

/// References
/// * [LocateHandle](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locatehandle)
#[must_use]
pub type LocateHandle = extern "efiapi" fn(
    LocateSearchType,
    *const Guid,
    *const Void,
    *mut usize,
    *mut crate::Handle,
) -> Status;

/// References
/// * [LocateHandleBuffer](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locatehandlebuffer)
#[must_use]
pub type LocateHandleBuffer = extern "efiapi" fn(
    LocateSearchType,
    *const Guid,
    *const Void,
    *mut usize,
    *mut *const crate::Handle,
) -> Status;

/// References
/// * [OpenProtocol](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-openprotocol)
#[must_use]
pub type Open =
    extern "efiapi" fn(Handle, *const Guid, *mut *const Void, Handle, Handle, u32) -> Status;

/// References
/// * [OpenProtocolInformation](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-openprotocolinformation)
#[must_use]
pub type OpenInformation =
    extern "efiapi" fn(Handle, *const Guid, *mut *const OpenInformationEntry, *mut usize) -> Status;

/// References
/// * [ProtocolsPerHandle](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-protocolsperhandle)
#[must_use]
pub type PerHandle = extern "efiapi" fn(Handle, *mut *const *const Guid, *mut usize) -> Status;

/// References
/// * [RegisterProtocolNotify](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-registerprotocolnotify)
#[must_use]
pub type RegisterNotify = extern "efiapi" fn(*const Guid, Event, *mut *const Void) -> Status;

/// References
/// * [ReinstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-reinstallprotocolinterface)
#[must_use]
pub type Reinstall =
    extern "efiapi" fn(crate::Handle, *const Guid, *const Void, *const Void) -> Status;

/// References
/// * [UninstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-uninstallprotocolinterface)
#[must_use]
pub type Uninstall = extern "efiapi" fn(crate::Handle, *const Guid, *const Void) -> Status;
