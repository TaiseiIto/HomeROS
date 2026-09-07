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

impl TryFrom<&[u8]> for Status {
    type Error = ();

    fn try_from(status: &[u8]) -> Result<Self, Self::Error> {
        str::from_utf8(status)
            .map_err(|_| ())
            .and_then(|status| match status {
                "okay" => Ok(Self::Okay),
                "disabled" => Ok(Self::Disabled),
                "reserved" => Ok(Self::Reserved),
                "fail" => Ok(Self::Fail(None)),
                status => status
                    .strip_prefix("fail-")
                    .map(|suffix| Self::Fail(Some(suffix.to_string())))
                    .ok_or(()),
            })
    }
}
