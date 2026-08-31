use {
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    core::{
        fmt::{Debug, Formatter, Result},
        mem::size_of,
    },
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3 Standard Properties
#[derive(Debug)]
pub enum Property {
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.5 #address-cells and #size-cells
    AddressCells(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.2 model
    ChassisType(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.1 compatible
    Compatible(Vec<String>),
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
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.9 dma-ranges
    DmaRanges(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.2 Properties for Interrupt Controllers
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
    InterruptCells(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.2 Properties for Interrupt Controllers
    InterruptController,
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
    InterruptMap(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
    InterruptMapMask(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
    Interrupts(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
    InterruptsExtended(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
    InterruptParent(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.2 model
    Model(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.12 name
    Name(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.2 /reserved-memory/ child nodes
    NoMap,
    Offset(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.3 phandle
    Phandle(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.8 ranges
    Ranges(Vec<u32>),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.6 reg
    Reg(Vec<u32>),
    RegMap(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.2 model
    SerialNumber(String),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.5 #address-cells and #size-cells
    SizeCells(u32),
    Unknown {
        name: String,
        data: Vec<u8>,
    },
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.4 status
    Status(String),
    Value(u32),
    /// # References
    /// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.7 virtual-reg
    VirtualReg(u32),
}

impl Property {
    pub fn new(name: &str, data: &[u8]) -> Self {
        match name {
            "#address-cells" => Self::AddressCells(u32::read(data)),
            "#interrupt-cells" => Self::InterruptCells(u32::read(data)),
            "#size-cells" => Self::SizeCells(u32::read(data)),
            "chassis-type" => Self::ChassisType(String::read(data)),
            "compatible" => Self::Compatible(Vec::<String>::read(data)),
            "device_type" => Self::DeviceType(String::read(data)),
            "dma-coherent" => Self::DmaCoherent,
            "dma-noncoherent" => Self::DmaNonCoherent,
            "dma-ranges" => Self::DmaRanges(Vec::<u32>::read(data)),
            "interrupt-controller" => Self::InterruptController,
            "interrupt-map" => Self::InterruptMap(Vec::<u32>::read(data)),
            "interrupt-map-mask" => Self::InterruptMapMask(Vec::<u32>::read(data)),
            "interrupt-parent" => Self::InterruptParent(u32::read(data)),
            "interrupts" => Self::Interrupts(Vec::<u32>::read(data)),
            "interrupts-extended" => Self::InterruptsExtended(Vec::<u32>::read(data)),
            "model" => Self::Model(String::read(data)),
            "name" => Self::Name(String::read(data)),
            "no-map" => Self::NoMap,
            "offset" => Self::Offset(u32::read(data)),
            "phandle" => Self::Phandle(u32::read(data)),
            "ranges" => Self::Ranges(Vec::<u32>::read(data)),
            "reg" => Self::Reg(Vec::<u32>::read(data)),
            "regmap" => Self::RegMap(u32::read(data)),
            "serial-number" => Self::SerialNumber(String::read(data)),
            "status" => Self::Status(String::read(data)),
            "value" => Self::Value(u32::read(data)),
            "virtual-reg" => Self::VirtualReg(u32::read(data)),
            name => Self::Unknown {
                name: name.to_string(),
                data: data.to_vec(),
            },
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

trait Reader {
    fn read(data: &[u8]) -> Self;
}

impl Reader for String {
    fn read(data: &[u8]) -> Self {
        str::from_utf8(&data[..data.len() - 1]).unwrap().to_string()
    }
}

impl Reader for Vec<u32> {
    fn read(data: &[u8]) -> Self {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<u32>() }>()
            .map(u32::from_be_bytes)
            .collect()
    }
}

impl Reader for Vec<String> {
    fn read(data: &[u8]) -> Self {
        Strings::new(data)
            .map(|string| string.to_string())
            .collect()
    }
}

impl Reader for u32 {
    fn read(data: &[u8]) -> Self {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<u32>() }>()
            .map(u32::from_be_bytes)
            .next()
            .unwrap()
    }
}
