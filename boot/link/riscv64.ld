ENTRY(_start)

SECTIONS {
	. = 0x80200000;
	.text : {
		*(.text._start)
		*(.text .text.*)
	}
	.rodata : {
		*(.rodata .rodata.*)
	}
	.data : {
		*(.data .data.*)
	}
	.bss : {
		*(.bss .bss.*)
	}
	_heap_head = .;
	. = ALIGN(4K);
	_stack_top = .;
	. += 0x40000;
	_stack_bottom = .;
}
