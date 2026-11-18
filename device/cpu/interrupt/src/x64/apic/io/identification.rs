/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.1 Identification Register (ID) - Offset 0h
#[io::register]
pub struct Register {
    __: [bool; 15],
    scratchpad: bool,
    __: u8,
    identification: [bool; 4],
    __: [bool; 4],
}
