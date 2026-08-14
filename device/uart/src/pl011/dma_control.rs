/// # References
/// * [DMA Control Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/dma-control-register--uartdmacr?lang=en)
#[bit::field]
pub struct Register {
    receive_dma_enable: bool,
    transmit_dma_enable: bool,
    dma_on_error: bool,
    __: [bool; 29],
}
