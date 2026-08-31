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
    fn data2cell(data: &[u8]) -> u32 {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<u32>() }>()
            .map(u32::from_be_bytes)
            .next()
            .unwrap()
    }

    fn data2cells(data: &[u8]) -> Vec<u32> {
        data.iter()
            .copied()
            .array_chunks::<{ size_of::<u32>() }>()
            .map(u32::from_be_bytes)
            .collect()
    }

    fn data2string(data: &[u8]) -> String {
        str::from_utf8(&data[..data.len() - 1]).unwrap().to_string()
    }

    fn data2strings(data: &[u8]) -> Vec<String> {
        Strings::new(data)
            .map(|string| string.to_string())
            .collect()
    }
}

impl Property {
    pub fn new(name: &str, data: &[u8]) -> Self {
        match name {
            "#address-cells" => Self::AddressCells(Self::data2cell(data)),
            "#interrupt-cells" => Self::InterruptCells(Self::data2cell(data)),
            "#size-cells" => Self::SizeCells(Self::data2cell(data)),
            "chassis-type" => Self::ChassisType(Self::data2string(data)),
            "compatible" => Self::Compatible(Self::data2strings(data)),
            "device_type" => Self::DeviceType(Self::data2string(data)),
            "dma-coherent" => Self::DmaCoherent,
            "dma-noncoherent" => Self::DmaNonCoherent,
            "dma-ranges" => Self::DmaRanges(Self::data2cells(data)),
            "interrupt-controller" => Self::InterruptController,
            "interrupt-map" => Self::InterruptMap(Self::data2cells(data)),
            "interrupt-map-mask" => Self::InterruptMapMask(Self::data2cells(data)),
            "interrupt-parent" => Self::InterruptParent(Self::data2cell(data)),
            "interrupts" => Self::Interrupts(Self::data2cells(data)),
            "interrupts-extended" => Self::InterruptsExtended(Self::data2cells(data)),
            "model" => Self::Model(Self::data2string(data)),
            "name" => Self::Name(Self::data2string(data)),
            "no-map" => Self::NoMap,
            "offset" => Self::Offset(Self::data2cell(data)),
            "phandle" => Self::Phandle(Self::data2cell(data)),
            "ranges" => Self::Ranges(Self::data2cells(data)),
            "reg" => Self::Reg(Self::data2cells(data)),
            "regmap" => Self::RegMap(Self::data2cell(data)),
            "serial-number" => Self::SerialNumber(Self::data2string(data)),
            "status" => Self::Status(Self::data2string(data)),
            "value" => Self::Value(Self::data2cell(data)),
            "virtual-reg" => Self::VirtualReg(Self::data2cell(data)),
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
