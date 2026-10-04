use alloc::string::{String, ToString};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.4 status
#[derive(Clone, Debug)]
pub enum Status {
    Okay,
    Disabled,
    Reserved,
    Fail(Option<String>),
}

impl TryFrom<&str> for Status {
    type Error = ();

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        match status {
            "okay" => Ok(Self::Okay),
            "disabled" => Ok(Self::Disabled),
            "reserved" => Ok(Self::Reserved),
            "fail" => Ok(Self::Fail(None)),
            status => status
                .strip_prefix("fail-")
                .map(|suffix| Self::Fail(Some(suffix.to_string())))
                .ok_or(()),
        }
    }
}
