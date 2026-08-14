/// # References
/// * [Interrupt FIFO Level Select Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/interrupt-fifo-level-select-register--uartifls?lang=en)
#[bit::field]
pub struct Register {
    transmit: [bool; 3],
    receive: [bool; 3],
    __: [bool; 26],
}
