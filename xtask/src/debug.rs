use {
    crate::{
        command::run,
        git::product,
        product::{Arch, Tree, Version},
        tmux,
    },
    regex::Automaton,
    std::{
        env::Args,
        fmt::{self, Display, Formatter},
        fs::{File, read},
        io::{BufRead, BufReader, Write},
        path::PathBuf,
    },
};

pub struct Command(Arch);

impl Command {
    const MEMORY: &str = "-m 1G";
    const REBOOT: &str = "--no-reboot";
    const VNC: &str = "-vnc :0";

    pub fn arch(&self) -> &Arch {
        &self.0
    }

    pub fn run(self) {
        if tmux::runs() {
            self.run_inside_tmux();
        } else {
            self.run_outside_tmux();
        }
    }

    fn boot(&self) -> String {
        match self.0 {
            Arch::RiscV64 => format!("-kernel {}", self.boot_destination().to_str().unwrap()),
            _ => String::new(),
        }
    }

    fn boot_destination(&self) -> PathBuf {
        let tree: Tree = self.into();
        tree.boot_destination()
    }

    fn com1(&self) -> String {
        let Self(arch) = self;
        match arch {
            Arch::X64 => format!("-serial file:{}", self.com1log().to_str().unwrap()),
            _ => format!(
                "-chardev stdio,id=com1,mux=on,logfile={} -serial chardev:com1",
                self.com1log().to_str().unwrap()
            ),
        }
    }

    fn com1log(&self) -> PathBuf {
        let mut com1log: PathBuf = self.log_directory();
        com1log.push("com1.log");
        com1log
    }

    fn com2(&self) -> String {
        let Self(arch) = self;
        match arch {
            Arch::X64 => format!(
                "-chardev stdio,id=com2,mux=on,logfile={} -serial chardev:com2",
                self.com2log().to_str().unwrap()
            ),
            _ => String::new(),
        }
    }

    fn com2log(&self) -> PathBuf {
        let mut com2log: PathBuf = self.log_directory();
        com2log.push("com2.log");
        com2log
    }
    fn cpu(&self) -> &str {
        let Self(arch) = self;
        match arch {
            Arch::Aarch64 => "-cpu max",
            _ => "",
        }
    }

    fn debug(&self) -> String {
        let Self(arch) = self;
        match arch {
            Arch::Aarch64 => format!("-serial file:{}", self.debug_log().to_str().unwrap()),
            Arch::X64 => format!(
                "-chardev file,id=debug,path={} -device isa-debugcon,iobase=0x402,chardev=debug -S -s",
                self.debug_log().to_str().unwrap(),
            ),
            _ => String::new(),
        }
    }

    fn debug_log(&self) -> PathBuf {
        let mut debug_log: PathBuf = self.log_directory();
        debug_log.push("debug.log");
        debug_log
    }

    fn destination(&self) -> PathBuf {
        let tree: Tree = self.into();
        tree.destination()
    }

    fn display(&self) -> &str {
        let Self(arch) = self;
        match arch {
            Arch::Aarch64 | Arch::RiscV64 => "-device ramfb",
            _ => "",
        }
    }

    fn drive(&self) -> String {
        match self.0 {
            Arch::Aarch64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device virtio-blk-device,drive={},bootindex=1",
                self.destination().to_str().unwrap(),
                product(),
                product()
            ),
            Arch::RiscV64 => format!(
                "-drive format=raw,file=fat:rw:{}",
                self.destination().to_str().unwrap(),
            ),
            Arch::X64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device ide-hd,drive={},bootindex=1",
                self.destination().to_str().unwrap(),
                product(),
                product()
            ),
        }
    }

    fn firmware(&self) -> String {
        match self.0 {
            Arch::Aarch64 => {
                format!("-drive if=pflash,format=raw,unit=0,file={},readonly=on", self.boot_destination().to_str().unwrap())
            }
            Arch::RiscV64 => "-bios default".to_string(),
            Arch::X64 => {
                "-drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_VARS.fd,format=raw,if=pflash".to_string()
            }
        }
    }

    fn log(&self) -> String {
        format!("-d int,cpu_reset -D {}", self.qemu_log().to_str().unwrap())
    }

    fn log_directory(&self) -> PathBuf {
        let mut log_directory: PathBuf = PathBuf::from("log");
        log_directory.push(match self.0 {
            Arch::Aarch64 => "aarch64",
            Arch::RiscV64 => "riscv64",
            Arch::X64 => "x64",
        });
        log_directory
    }

    fn machine(&self) -> &str {
        let Self(arch) = self;
        match arch {
            Arch::Aarch64 => "-machine virt,secure=on",
            Arch::RiscV64 => "-machine virt",
            Arch::X64 => "",
        }
    }

    fn new(arch: Arch) -> Self {
        Self(arch)
    }

    fn qemu(&self) -> &str {
        let Self(arch) = self;
        match arch {
            Arch::Aarch64 => "qemu-system-aarch64",
            Arch::RiscV64 => "qemu-system-riscv64",
            Arch::X64 => "qemu-system-x86_64",
        }
    }

    fn qemu_log(&self) -> PathBuf {
        let mut qemu_log: PathBuf = self.log_directory();
        qemu_log.push("qemu.log");
        qemu_log
    }

    fn run_inside_tmux(self) {
        run(&format!("mkdir -p {:?}", self.log_directory()));
        run(&self.to_string());
    }

    fn run_outside_tmux(self) {
        let debug_log: File = File::open(self.debug_log()).unwrap();
        let debug_log: BufReader<File> = BufReader::new(debug_log);
        let automaton: Automaton =
            r"^Loading driver at 0x(?<address>[\da-fA-F]+) EntryPoint=0x[\da-fA-F]+ boot\.efi$"
                .parse()
                .unwrap();
        let boot_loader_address: usize = debug_log
            .lines()
            .map_while(Result::ok)
            .find_map(|line| {
                automaton.input(&line).into_iter().next().and_then(|mat| {
                    mat.captures()["address"]
                        .first()
                        .and_then(|address| usize::from_str_radix(address.into(), 16).ok())
                })
            })
            .unwrap();
        let boot_destination: PathBuf = self.boot_destination();
        let boot: Vec<u8> = read(&boot_destination).unwrap();
        let pe: pe::File = boot.as_slice().into();
        let first_section_offset: usize = pe.first_section_offset();
        let boot_loader_address: usize = boot_loader_address + first_section_offset;
        let architecture: &str = match self.arch() {
            Arch::Aarch64 => "aarch64",
            Arch::RiscV64 => "riscv:rv64",
            Arch::X64 => "i386:x86-64",
        };
        let mut gdbinit = File::create(".gdbinit").unwrap();
        write!(
            gdbinit,
            r#"
set architecture {}
file {:?}
target remote localhost:1234
add-symbol-file {:?} {:#x?}
break boot::efi_main
continue
"#,
            architecture, boot_destination, boot_destination, boot_loader_address
        )
        .unwrap();
        gdbinit.flush().unwrap();
        // let Self(arch) = self;
        // let source: PathBuf = PathBuf::from(".docker/tmux/dbg");
        // run(&format!(
        //     "ARCH={} VERSION=debug tmux new-session ; source-file {}",
        //     arch,
        //     source.to_str().unwrap()
        // ));
    }
}

impl From<&Command> for Tree {
    fn from(command: &Command) -> Self {
        let Command(arch) = command;
        Self::new(arch.clone(), Version::Debug)
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let mut arch: Option<Arch> = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--arch" => arch = Some(args.next().unwrap().as_str().into()),
                arg => unreachable!("arg = {}", arg),
            }
        }
        Self::new(arch.unwrap())
    }
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            &[
                self.qemu(),
                &self.boot(),
                &self.com1(),
                &self.com2(),
                self.cpu(),
                &self.debug(),
                self.display(),
                &self.drive(),
                &self.firmware(),
                &self.log(),
                self.machine(),
                Self::MEMORY,
                Self::REBOOT,
                Self::VNC,
            ]
            .join(" "),
        )
    }
}
