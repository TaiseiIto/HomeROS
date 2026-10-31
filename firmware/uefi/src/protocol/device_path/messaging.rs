mod atapi;
mod bluetooth;
mod device1394;
mod device_logical_unit;
mod emmc;
mod fibre;
mod i2o;
mod infini_band;
mod ip;
mod iscsi;
mod mac_address;
mod nvm_express_namespace;
mod sas_ex;
mod sata;
mod scsi;
mod sd;
mod uart;
mod ufs;
mod uri;
mod usb;
mod vendor;
mod vlan;
mod wifi;

use {
    super::{Node, Type},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Messaging Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#messaging-device-path)
/// # TODO
/// * Print all subtypes.
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match self.sub_type() {
            SubType::Atapi => unsafe { &*(self as *const Self as *const atapi::Protocol) },
            SubType::Scsi => unsafe { &*(self as *const Self as *const scsi::Protocol) },
            SubType::Fibre => unsafe { &*(self as *const Self as *const fibre::Protocol) },
            SubType::FibreEx => unsafe { &*(self as *const Self as *const fibre::ex::Protocol) },
            SubType::Device1394 => unsafe {
                &*(self as *const Self as *const device1394::Protocol)
            },
            SubType::Usb => unsafe { &*(self as *const Self as *const usb::Protocol) },
            SubType::Sata => unsafe { &*(self as *const Self as *const sata::Protocol) },
            SubType::UsbWwid => unsafe { &*(self as *const Self as *const usb::wwid::Protocol) },
            SubType::DeviceLogicalUnit => unsafe {
                &*(self as *const Self as *const device_logical_unit::Protocol)
            },
            SubType::UsbClass => unsafe { &*(self as *const Self as *const usb::class::Protocol) },
            SubType::I2O => unsafe { &*(self as *const Self as *const i2o::Protocol) },
            SubType::MacAddress => unsafe {
                &*(self as *const Self as *const mac_address::Protocol)
            },
            SubType::Ipv4 => unsafe { &*(self as *const Self as *const ip::v4::Protocol) },
            SubType::Ipv6 => unsafe { &*(self as *const Self as *const ip::v6::Protocol) },
            SubType::Vlan => unsafe { &*(self as *const Self as *const vlan::Protocol) },
            SubType::InfiniBand => unsafe {
                &*(self as *const Self as *const infini_band::Protocol)
            },
            SubType::Uart => unsafe { &*(self as *const Self as *const uart::Protocol) },
            SubType::Vendor => unsafe { &*(self as *const Self as *const vendor::Protocol) },
            SubType::SasEx => unsafe { &*(self as *const Self as *const sas_ex::Protocol) },
            SubType::Iscsi => unsafe { &*(self as *const Self as *const iscsi::Protocol) },
            SubType::NvmExpressNamespace => unsafe {
                &*(self as *const Self as *const nvm_express_namespace::Protocol)
            },
            SubType::Uri => unsafe { &*(self as *const Self as *const uri::Protocol) },
            SubType::Ufs => unsafe { &*(self as *const Self as *const ufs::Protocol) },
            SubType::Sd => unsafe { &*(self as *const Self as *const sd::Protocol) },
            SubType::Bluetooth => unsafe { &*(self as *const Self as *const bluetooth::Protocol) },
            SubType::Wifi => unsafe { &*(self as *const Self as *const wifi::Protocol) },
            SubType::Emmc => unsafe { &*(self as *const Self as *const emmc::Protocol) },
            _ => unimplemented!(),
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.read().fmt(formatter)
    }
}

impl Node for Protocol {}

/// # References
/// * [Messaging Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#messaging-device-path)
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum SubType {
    Atapi = 1,
    Scsi = 2,
    Fibre = 3,
    FibreEx = 21,
    Device1394 = 4,
    Usb = 5,
    Sata = 18,
    UsbWwid = 16,
    DeviceLogicalUnit = 17,
    UsbClass = 15,
    I2O = 6,
    MacAddress = 11,
    Ipv4 = 12,
    Ipv6 = 13,
    Vlan = 20,
    InfiniBand = 9,
    Uart = 14,
    Vendor = 10,
    SasEx = 22,
    Iscsi = 19,
    NvmExpressNamespace = 23,
    Uri = 24,
    Ufs = 25,
    Sd = 26,
    Bluetooth = 27,
    Wifi = 28,
    Emmc = 29,
    BluetoothLe = 30,
    Dns = 31,
    Nvdimm = 32,
    RestService = 33,
    NvmeofNamespace = 34,
}
