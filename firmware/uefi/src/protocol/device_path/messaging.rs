mod atapi;
mod bluetooth;
mod device1394;
mod device_logical_unit;
mod dns;
mod emmc;
mod fibre;
mod i2o;
mod infini_band;
mod ip;
mod iscsi;
mod mac_address;
mod nvdimm;
mod nvm_express_namespace;
mod nvmeof_namespace;
mod rest_service;
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
    super::Node,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Messaging Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#messaging-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
}

impl Protocol {
    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn read(&self) -> &dyn Node {
        match self.get_sub_type() {
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
            SubType::BluetoothLe => unsafe {
                &*(self as *const Self as *const bluetooth::le::Protocol)
            },
            SubType::Dns => unsafe { &*(self as *const Self as *const dns::Protocol) },
            SubType::Nvdimm => unsafe { &*(self as *const Self as *const nvdimm::Protocol) },
            SubType::RestService => unsafe {
                &*(self as *const Self as *const rest_service::Protocol)
            },
            SubType::NvmeofNamespace => unsafe {
                &*(self as *const Self as *const nvmeof_namespace::Protocol)
            },
            SubType::Unknown(_) => panic!(),
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
enum SubType {
    Atapi,
    Scsi,
    Fibre,
    FibreEx,
    Device1394,
    Usb,
    Sata,
    UsbWwid,
    DeviceLogicalUnit,
    UsbClass,
    I2O,
    MacAddress,
    Ipv4,
    Ipv6,
    Vlan,
    InfiniBand,
    Uart,
    Vendor,
    SasEx,
    Iscsi,
    NvmExpressNamespace,
    Uri,
    Ufs,
    Sd,
    Bluetooth,
    Wifi,
    Emmc,
    BluetoothLe,
    Dns,
    Nvdimm,
    RestService,
    NvmeofNamespace,
    Unknown(u8),
}

impl From<u8> for SubType {
    fn from(sub_type: u8) -> Self {
        match sub_type {
            1 => Self::Atapi,
            2 => Self::Scsi,
            3 => Self::Fibre,
            21 => Self::FibreEx,
            4 => Self::Device1394,
            5 => Self::Usb,
            18 => Self::Sata,
            16 => Self::UsbWwid,
            17 => Self::DeviceLogicalUnit,
            15 => Self::UsbClass,
            6 => Self::I2O,
            11 => Self::MacAddress,
            12 => Self::Ipv4,
            13 => Self::Ipv6,
            20 => Self::Vlan,
            9 => Self::InfiniBand,
            14 => Self::Uart,
            10 => Self::Vendor,
            22 => Self::SasEx,
            19 => Self::Iscsi,
            23 => Self::NvmExpressNamespace,
            24 => Self::Uri,
            25 => Self::Ufs,
            26 => Self::Sd,
            27 => Self::Bluetooth,
            28 => Self::Wifi,
            29 => Self::Emmc,
            30 => Self::BluetoothLe,
            31 => Self::Dns,
            32 => Self::Nvdimm,
            33 => Self::RestService,
            34 => Self::NvmeofNamespace,
            sub_type => Self::Unknown(sub_type),
        }
    }
}
