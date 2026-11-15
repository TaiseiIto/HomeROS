use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Windows ACPI Emulated Devices Table](https://learn.microsoft.com/en-us/previous-versions/gg487524(v=msdn.10)?redirectedfrom=MSDN)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    flags: FlagsRaw,
}

impl Table {
    fn get_flags(&self) -> Flags {
        self.flags().into()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("falgs", &self.get_flags())
            .finish()
    }
}

impl super::Table for Table {}

#[io::register]
struct Flags {
    rtc_good: bool,
    pm_timer_good: bool,
    __: [bool; 30],
}
