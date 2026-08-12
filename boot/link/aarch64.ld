ENTRY(_start)

SECTIONS {
	. = 0x60000000;
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
	. = ALIGN(4K);
	_stack_top = .;
	. += 0x10000;
	_stack_bottom = .;
	/DISCARD/ : {
		*(.comment)
		*(.debug*)
		*(.eh_frame*)
	}
}
