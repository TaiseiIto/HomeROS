use {
    alloc::string::{String, ToString},
    core::str::Split,
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.1 compatible
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 4.2.1 Serial Class Binding
#[derive(Clone, Debug)]
pub struct Compatible {
    vendor: Option<String>,
    device: String,
}

impl From<&str> for Compatible {
    fn from(compatible: &str) -> Self {
        let mut compatible: Split<'_, char> = compatible.split(',');
        let first_word: &str = compatible.next().unwrap();
        let second_word: Option<&str> = compatible.next();
        if let Some(device) = second_word {
            Self {
                vendor: Some(first_word.to_string()),
                device: device.to_string(),
            }
        } else {
            Self {
                vendor: None,
                device: first_word.to_string(),
            }
        }
    }
}
