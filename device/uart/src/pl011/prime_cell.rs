/// # References
/// * [PrimeCell Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/primecell-identification-registers--uartpcellid0-3?lang=en)
#[bit::field]
pub struct Id0 {
    id: u8,
    __: [u8; 3],
}

/// # References
/// * [PrimeCell Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/primecell-identification-registers--uartpcellid0-3?lang=en)
#[bit::field]
pub struct Id1 {
    id: u8,
    __: [u8; 3],
}

/// # References
/// * [PrimeCell Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/primecell-identification-registers--uartpcellid0-3?lang=en)
#[bit::field]
pub struct Id2 {
    id: u8,
    __: [u8; 3],
}

/// # References
/// * [PrimeCell Identification Registers](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/primecell-identification-registers--uartpcellid0-3?lang=en)
#[bit::field]
pub struct Id3 {
    id: u8,
    __: [u8; 3],
}
