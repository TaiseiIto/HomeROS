/// # References
/// * [Peripheral Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/peripheral-identification-registers--uartperiphid0-3?lang=en)
#[io::register]
pub struct Id0 {
    part_number0: u8,
    __: [u8; 3],
}

/// # References
/// * [Peripheral Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/peripheral-identification-registers--uartperiphid0-3?lang=en)
#[io::register]
pub struct Id1 {
    part_number1: [bool; 4],
    designer0: [bool; 4],
    __: [u8; 3],
}

/// # References
/// * [Peripheral Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/peripheral-identification-registers--uartperiphid0-3?lang=en)
#[io::register]
pub struct Id2 {
    designer1: [bool; 4],
    revision: [bool; 4],
    __: [u8; 3],
}

/// # References
/// * [Peripheral Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/peripheral-identification-registers--uartperiphid0-3?lang=en)
#[io::register]
pub struct Id3 {
    configuration: u8,
    __: [u8; 3],
}
