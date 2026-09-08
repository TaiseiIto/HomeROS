mod alignment;
mod alloc_ranges;
mod clocks;
mod compatible;
mod dma;
mod interrupt;
mod map;
mod ranges;
mod reg;
mod standard;
pub mod status;

use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alignment::Alignment,
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    alloc_ranges::AllocRanges,
    clocks::Clocks,
    compatible::Compatible,
    core::{
        fmt::{Debug, Formatter, Result},
        mem::size_of,
    },
    map::Map,
    ranges::Ranges,
    reg::Reg,
    status::Status,
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3 Standard Properties
#[derive(Clone, Debug)]
pub enum Property {
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.1 Network Class Binding
    AddressBits(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.5 #address-cells and #size-cells
    AddressCells(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.2 /reserved-memory/ child nodes
    Alignment(Alignment),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.2 /reserved-memory/ child nodes
    AllocRanges(AllocRanges),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.6 /chosen Node
    BootArgs(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    CacheBlockSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.9 Multi-level and Shared Cache Nodes (/cpu/cpu*/l?-cache)
    CacheLevel(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    CacheLineSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    CacheOpBlockSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    CacheSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    CacheSets(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    CacheUnified,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.5.1 Nexus Node Properties
    Cells {
        specifier: String,
        cells: u32,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.2 model
    ChassisType(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.1.2 Miscellaneous Properties
    ClockFrequency(u64),
    Clocks(Clocks),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.1 compatible
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.2.1 Serial Class Binding
    Compatible(Vec<Compatible>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    CpuReleaseAddr(u64),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.2.2 National Semiconductor 16450/16550 Compatible UART Requirements
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.2.1 Serial Class Binding
    CurrentSpeed(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.9 dma-ranges
    DmaRanges(dma::Ranges),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    DCacheSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    DCacheSets(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    DCacheBlockSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    DCacheLineSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    DTlbSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    DTlbSets(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.13 device_type (deprecated)
    DeviceType(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.10 dma-coherent
    DmaCoherent,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.11 dma-noncoherent
    DmaNonCoherent,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    EnableMethod(Vec<String>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.4 /memory node
    HotPluggable,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.4 /memory node
    InitialMappedArea {
        effective_address: u64,
        physical_address: u64,
        size: u32,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    ICacheSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    ICacheSets(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    ICacheBlockSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    ICacheLineSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.2 Properties for Interrupt Controllers
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
    InterruptCells(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.2 Properties for Interrupt Controllers
    InterruptController,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
    InterruptMap(interrupt::Map),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
    InterruptMapMask(interrupt::map::Mask),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
    Interrupts(interrupt::Specifiers),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
    InterruptsExtended(interrupt::Extended),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
    InterruptParent(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    ITlbSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    ITlbSets(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.1.2 Miscellaneous Properties
    Label(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.1 Network Class Binding
    LocalMacAddress([u8; 6]),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.1 Network Class Binding
    MacAddress([u8; 6]),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.5.1 Nexus Node Properties
    Map {
        specifier: String,
        map: Map,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.5.1 Nexus Node Properties
    MapMask {
        specifier: String,
        map_mask: map::Mask,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.5.1 Nexus Node Properties
    MapPassThru {
        specifier: String,
        map_mask: map::Mask,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.1 Network Class Binding
    MaxFrameSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.2 Ethernet specific considerations
    MaxSpeed(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.3 Device node references to reserved memory
    MemoryRegion(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.3 Device node references to reserved memory
    MemoryRegionNames(Vec<String>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    MmuType(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.2 model
    Model(String),
    MsiMap {
        rid_base: u32,
        msi_parent: u32,
        msi_base: u128,
        msi_length: u32,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.12 name
    Name(String),
    Names {
        specifier: String,
        names: Vec<String>,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.3 Internal (L1) Cache Properties
    NextLevelCache(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.5 simple-bus Compatible Value
    NonPostedMmio,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.2 /reserved-memory/ child nodes
    NoMap,
    Offset(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.3 phandle
    PHandle(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.2 Ethernet specific considerations
    PhyConnectionType(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.3.2 Ethernet specific considerations
    PhyHandle(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    PowerIsa {
        cat: String,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    PowerIsaVersion(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.8 ranges
    Ranges(Ranges),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.6 reg
    Reg(Reg),
    RegMap(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.1.2 Miscellaneous Properties
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.2.2 National Semiconductor 16450/16550 Compatible UART Requirements
    RegShift(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    ReservationGranuleSiz(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.2 /reserved-memory/ child nodes
    Reusable,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.2 model
    SerialNumber(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.5 #address-cells and #size-cells
    SizeCells(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.4 status
    Status(Status),
    StatusWithSpecifier {
        specifier: String,
        status: Status,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.6 /chosen Node
    StdInPath(standard::Path),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.6 /chosen Node
    StdOutPath(standard::Path),
    Unknown {
        name: String,
        data: Vec<u8>,
    },
    UnknownStrings {
        name: String,
        strings: Vec<String>,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.1 General Properties of /cpus/cpu* nodes
    TimeBaseFrequency(u64),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    TlbSets(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    TlbSize(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.8.2 TLB Properties
    TlbSplit,
    Value(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.7 virtual-reg
    VirtualReg(u64),
}

impl Property {
    pub fn new(name: &str, data: &[u8]) -> Self {
        match name {
            "#address-cells" => Self::AddressCells(u32::read(data)),
            "#interrupt-cells" => Self::InterruptCells(u32::read(data)),
            "#size-cells" => Self::SizeCells(u32::read(data)),
            "address-bits" => Self::AddressBits(u32::read(data)),
            "alignment" => Self::Alignment(Alignment::Raw(Vec::<u32>::read(data))),
            "alloc-ranges" => Self::AllocRanges(AllocRanges::Raw(Vec::<u32>::read(data))),
            "bootargs" => Self::BootArgs(String::read(data)),
            "cache-block-size" => Self::CacheBlockSize(u32::read(data)),
            "cache-level" => Self::CacheLevel(u32::read(data)),
            "cache-line-size" => Self::CacheLineSize(u32::read(data)),
            "cache-op-block-size" => Self::CacheOpBlockSize(u32::read(data)),
            "cache-sets" => Self::CacheSets(u32::read(data)),
            "cache-size" => Self::CacheSize(u32::read(data)),
            "cache-unified" => Self::CacheUnified,
            "chassis-type" => Self::ChassisType(String::read(data)),
            "clock-frequency" => Self::ClockFrequency(match data.len() {
                4 => u32::read(data) as u64,
                8 => u64::read(data),
                _ => panic!(),
            }),
            "clocks" => Self::Clocks(Clocks::Raw(Vec::<u32>::read(data))),
            "compatible" => Self::Compatible(
                Vec::<&str>::read(data)
                    .into_iter()
                    .map(|compatible| compatible.into())
                    .collect(),
            ),
            "cpu-release-addr" => Self::CpuReleaseAddr(u64::read(data)),
            "current-speed" => Self::CurrentSpeed(u32::read(data)),
            "d-cache-block-size" => Self::DCacheBlockSize(u32::read(data)),
            "d-cache-line-size" => Self::DCacheLineSize(u32::read(data)),
            "d-cache-sets" => Self::DCacheSets(u32::read(data)),
            "d-cache-size" => Self::DCacheSize(u32::read(data)),
            "d-tlb-sets" => Self::DTlbSets(u32::read(data)),
            "d-tlb-size" => Self::DTlbSize(u32::read(data)),
            "device_type" => Self::DeviceType(String::read(data)),
            "dma-coherent" => Self::DmaCoherent,
            "dma-noncoherent" => Self::DmaNonCoherent,
            "dma-ranges" => Self::DmaRanges(dma::Ranges::Raw(Vec::<u32>::read(data))),
            "enable-method" => Self::EnableMethod(Vec::<String>::read(data)),
            "hotpluggable" => Self::HotPluggable,
            "initial-mapped-area" => Self::InitialMappedArea {
                effective_address: u64::read(data),
                physical_address: u64::read(&data[size_of::<u64>()..]),
                size: u32::read(&data[2 * size_of::<u64>()..]),
            },
            "interrupt-controller" => Self::InterruptController,
            "interrupt-map" => Self::InterruptMap(interrupt::Map::Raw(Vec::<u32>::read(data))),
            "interrupt-map-mask" => {
                Self::InterruptMapMask(interrupt::map::Mask::Raw(Vec::<u32>::read(data)))
            }
            "interrupt-parent" => Self::InterruptParent(u32::read(data)),
            "interrupts" => Self::Interrupts(interrupt::Specifiers::Raw(Vec::<u32>::read(data))),
            "interrupts-extended" => {
                Self::InterruptsExtended(interrupt::Extended::Raw(Vec::<u32>::read(data)))
            }
            "i-cache-block-size" => Self::ICacheBlockSize(u32::read(data)),
            "i-cache-line-size" => Self::ICacheLineSize(u32::read(data)),
            "i-cache-sets" => Self::ICacheSets(u32::read(data)),
            "i-cache-size" => Self::ICacheSize(u32::read(data)),
            "i-tlb-sets" => Self::ITlbSets(u32::read(data)),
            "i-tlb-size" => Self::ITlbSize(u32::read(data)),
            "label" => Self::Label(String::read(data)),
            "local-mac-address" => Self::LocalMacAddress(<[u8; 6]>::read(data)),
            "mac-address" => Self::MacAddress(<[u8; 6]>::read(data)),
            "max-frame-size" => Self::MaxFrameSize(u32::read(data)),
            "max-speed" => Self::MaxSpeed(u32::read(data)),
            "memory-region" => Self::MemoryRegion(Vec::<u32>::read(data)),
            "memory-region-names" => Self::MemoryRegionNames(Vec::<String>::read(data)),
            "mmu-type" => Self::MmuType(String::read(data)),
            "model" => Self::Model(String::read(data)),
            "msi-map" => {
                let data: Vec<u32> = Vec::<u32>::read(data);
                let (rid_base, data): (&u32, &[u32]) = data.as_slice().split_first().unwrap();
                let (msi_parent, data): (&u32, &[u32]) = data.split_first().unwrap();
                let (msi_length, data): (&u32, &[u32]) = data.split_last().unwrap();
                let msi_base: u128 = data
                    .iter()
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                Self::MsiMap {
                    rid_base: *rid_base,
                    msi_parent: *msi_parent,
                    msi_base,
                    msi_length: *msi_length,
                }
            }
            "name" => Self::Name(String::read(data)),
            "next-level-cache" => Self::NextLevelCache(u32::read(data)),
            "no-map" => Self::NoMap,
            "nonposted-mmio" => Self::NonPostedMmio,
            "offset" => Self::Offset(u32::read(data)),
            "phandle" => Self::PHandle(u32::read(data)),
            "phy-connection-type" => Self::PhyConnectionType(String::read(data)),
            "phy-handle" => Self::PhyHandle(u32::read(data)),
            "power-isa-version" => Self::PowerIsaVersion(String::read(data)),
            "ranges" => Self::Ranges(Ranges::Raw(Vec::<u32>::read(data))),
            "reg" => Self::Reg(Reg::Raw(Vec::<u32>::read(data))),
            "reg-shift" => Self::RegShift(u32::read(data)),
            "regmap" => Self::RegMap(u32::read(data)),
            "reservation-granule-siz" => Self::ReservationGranuleSiz(u32::read(data)),
            "reusable" => Self::Reusable,
            "serial-number" => Self::SerialNumber(String::read(data)),
            "status" => Self::Status(String::read(data).as_str().try_into().unwrap()),
            "stdin-path" => Self::StdInPath(<&str>::read(data).into()),
            "stdout-path" => Self::StdOutPath(<&str>::read(data).into()),
            "timebase-frequency" => Self::TimeBaseFrequency(match data.len() {
                4 => u32::read(data) as u64,
                8 => u64::read(data),
                _ => panic!(),
            }),
            "tlb-sets" => Self::TlbSets(u32::read(data)),
            "tlb-size" => Self::TlbSize(u32::read(data)),
            "tlb-split" => Self::TlbSplit,
            "value" => Self::Value(u32::read(data)),
            "virtual-reg" => Self::VirtualReg(match data.len() {
                4 => u32::read(data) as u64,
                8 => u64::read(data),
                _ => panic!(),
            }),
            name => {
                if let Some(specifier) = name.strip_suffix("-map") {
                    Self::Map {
                        specifier: specifier.to_string(),
                        map: Map::Raw(Vec::<u32>::read(data)),
                    }
                } else if let Some(specifier) = name.strip_suffix("-map-mask") {
                    Self::MapMask {
                        specifier: specifier.to_string(),
                        map_mask: map::Mask::Raw(Vec::<u32>::read(data)),
                    }
                } else if let Some(specifier) = name.strip_suffix("-map-pass-thru") {
                    Self::MapPassThru {
                        specifier: specifier.to_string(),
                        map_mask: map::Mask::Raw(Vec::<u32>::read(data)),
                    }
                } else if let Some(specifier) = name.strip_suffix("-names") {
                    Self::Names {
                        specifier: specifier.to_string(),
                        names: Vec::<String>::read(data),
                    }
                } else if let Some(specifier) = name.strip_suffix("-status") {
                    Self::StatusWithSpecifier {
                        specifier: specifier.to_string(),
                        status: String::read(data).as_str().try_into().unwrap(),
                    }
                } else if let Some(name) = name.strip_prefix("#")
                    && let Some(specifier) = name.strip_suffix("-cells")
                {
                    Self::Cells {
                        specifier: specifier.to_string(),
                        cells: u32::read(data),
                    }
                } else if let Some(cat) = name.strip_prefix("power-isa-") {
                    Self::PowerIsa {
                        cat: cat.to_string(),
                    }
                } else if data.iter().all(|byte| *byte == 0x00 || byte.is_ascii()) {
                    Self::UnknownStrings {
                        name: name.to_string(),
                        strings: Vec::<String>::read(data),
                    }
                } else {
                    Self::Unknown {
                        name: name.to_string(),
                        data: data.to_vec(),
                    }
                }
            }
        }
    }
}

impl SecondAnalyzed for Property {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        match self {
            Self::Alignment(alignment) => {
                Self::Alignment(second_analyzer.second_analyze(alignment))
            }
            Self::AllocRanges(alloc_ranges) => {
                Self::AllocRanges(second_analyzer.second_analyze(alloc_ranges))
            }
            Self::Clocks(clocks) => Self::Clocks(second_analyzer.second_analyze(clocks)),
            Self::DmaRanges(dma_ranges) => {
                Self::DmaRanges(second_analyzer.second_analyze(dma_ranges))
            }
            Self::InterruptMap(interrupt_map) => {
                Self::InterruptMap(second_analyzer.second_analyze(interrupt_map))
            }
            Self::InterruptMapMask(interrupt_map_mask) => {
                Self::InterruptMapMask(second_analyzer.second_analyze(interrupt_map_mask))
            }
            Self::Interrupts(interrupts) => {
                Self::Interrupts(second_analyzer.second_analyze(interrupts))
            }
            Self::InterruptsExtended(interrupts_extended) => {
                Self::InterruptsExtended(second_analyzer.second_analyze(interrupts_extended))
            }
            Self::Map { specifier, map } => Self::Map {
                specifier: specifier.clone(),
                map: second_analyzer.second_analyze_with_specifier(map, specifier.as_str()),
            },
            Self::MapMask {
                specifier,
                map_mask,
            } => Self::MapMask {
                specifier: specifier.clone(),
                map_mask: second_analyzer.second_analyze(map_mask),
            },
            Self::MapPassThru {
                specifier,
                map_mask,
            } => Self::MapPassThru {
                specifier: specifier.clone(),
                map_mask: second_analyzer.second_analyze(map_mask),
            },
            Self::Ranges(ranges) => Self::Ranges(second_analyzer.second_analyze(ranges)),
            Self::Reg(reg) => Self::Reg(second_analyzer.second_analyze(reg)),
            _ => self.clone(),
        }
    }
}

#[derive(Clone)]
struct Strings<'a> {
    strings: &'a [u8],
    offset: usize,
}

impl<'a> Strings<'a> {
    fn new(strings: &'a [u8]) -> Self {
        Self { strings, offset: 0 }
    }
}

impl Debug for Strings<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a> Iterator for Strings<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { strings, offset } = self;
        let strings_length: usize = strings.len();
        let begin: usize = *offset;
        if begin < strings_length {
            let end: usize = (begin..strings_length)
                .take_while(|offset| strings.get(*offset).is_some_and(|byte| *byte != 0x00))
                .max()
                .map(|last_index| last_index + 1)
                .unwrap_or(begin);
            *offset += end - begin + 1;
            Some(str::from_utf8(&strings[begin..end]).unwrap())
        } else {
            None
        }
    }
}

trait Reader<'a> {
    fn read(data: &'a [u8]) -> Self;
}

impl<const N: usize> Reader<'_> for [u8; N] {
    fn read(data: &[u8]) -> Self {
        data.iter().copied().array_chunks::<N>().next().unwrap()
    }
}

impl Reader<'_> for String {
    fn read(data: &[u8]) -> Self {
        <&str>::read(data).to_string()
    }
}

impl<'a> Reader<'a> for &'a str {
    fn read(data: &'a [u8]) -> Self {
        str::from_utf8(&data[..data.len() - 1]).unwrap()
    }
}

impl Reader<'_> for Vec<u8> {
    fn read(data: &[u8]) -> Self {
        data.to_vec()
    }
}

impl Reader<'_> for Vec<u32> {
    fn read(data: &[u8]) -> Self {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<u32>() }>()
            .map(u32::from_be_bytes)
            .collect()
    }
}

impl Reader<'_> for Vec<String> {
    fn read(data: &[u8]) -> Self {
        Strings::new(data)
            .map(|string| string.to_string())
            .collect()
    }
}

impl<'a> Reader<'a> for Vec<&'a str> {
    fn read(data: &'a [u8]) -> Self {
        Strings::new(data).collect()
    }
}

impl Reader<'_> for u32 {
    fn read(data: &[u8]) -> Self {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<Self>() }>()
            .map(Self::from_be_bytes)
            .next()
            .unwrap()
    }
}

impl Reader<'_> for u64 {
    fn read(data: &[u8]) -> Self {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<Self>() }>()
            .map(Self::from_be_bytes)
            .next()
            .unwrap()
    }
}
