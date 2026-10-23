use {
    crate::{
        command::run,
        git::product,
        product::{Arch, Binary, Tree, Version},
    },
    regex::Automaton,
    std::{
        collections::HashMap,
        env::Args,
        fmt::{self, Display, Formatter},
        fs::{read, read_to_string, remove_file, write},
        path::PathBuf,
        thread::sleep,
        time::Duration,
    },
};

pub struct Command(Arch);

impl Command {
    pub fn arch(&self) -> &Arch {
        &self.0
    }

    pub fn run(self) {
        let source: PathBuf = PathBuf::from(".docker/tmux/dbg");
        run(&format!(
            "ARCH={} tmux new-session ; source-file {}",
            self.arch(),
            source.to_str().unwrap()
        ));
    }

    fn new(arch: Arch) -> Self {
        Self(arch)
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

pub struct Debuggee(Arch);

impl Debuggee {
    const MEMORY: &str = "-m 1G";
    const REBOOT: &str = "--no-reboot";
    const VNC: &str = "-vnc :0";

    pub fn arch(&self) -> &Arch {
        &self.0
    }

    pub fn run(self) {
        run(&format!("mkdir -p {:?}", self.log_directory()));
        run(&self.to_string());
    }

    fn boot(&self) -> Binary {
        let tree: Tree = self.into();
        tree.boot()
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
            Arch::Aarch64 => format!("-serial file:{} -s -S", self.debug_log().to_str().unwrap()),
            Arch::X64 => format!(
                "-chardev file,id=debug,path={} -device isa-debugcon,iobase=0x402,chardev=debug -s",
                self.debug_log().to_str().unwrap(),
            ),
            _ => "-s -S".to_string(),
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
                format!("-drive if=pflash,format=raw,unit=0,file={},readonly=on", self.boot().destination().to_str().unwrap())
            }
            Arch::RiscV64 => "-bios default".to_string(),
            Arch::X64 => {
                "-drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_VARS.fd,format=raw,if=pflash".to_string()
            }
        }
    }

    fn kernel(&self) -> String {
        match self.0 {
            Arch::RiscV64 => format!("-kernel {}", self.boot().destination().to_str().unwrap()),
            _ => String::new(),
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
}

impl From<&Debugger> for Debuggee {
    fn from(debugger: &Debugger) -> Self {
        Self::new(debugger.arch().clone())
    }
}

impl From<&Debuggee> for Tree {
    fn from(debuggee: &Debuggee) -> Self {
        let Debuggee(arch) = debuggee;
        Self::new(arch.clone(), Version::Debug)
    }
}

impl From<Args> for Debuggee {
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

impl Display for Debuggee {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            &[
                self.qemu(),
                &self.com1(),
                &self.com2(),
                self.cpu(),
                &self.debug(),
                self.display(),
                &self.drive(),
                &self.firmware(),
                &self.kernel(),
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

#[allow(dead_code)]
pub struct Debugger(Arch);

impl Debugger {
    pub fn run(self) {
        let gdbinit: PathBuf = PathBuf::from(".gdbinit");
        if gdbinit.exists() {
            assert!(gdbinit.is_file());
            remove_file(&gdbinit).unwrap();
        }
        let arch: &str = match self.arch() {
            Arch::Aarch64 => "aarch64",
            Arch::RiscV64 => "riscv:rv64",
            Arch::X64 => "i386:x86-64",
        };
        let boot_loader: PathBuf = self.boot().source();
        let option: String = match self.arch() {
            Arch::X64 => {
                let boot_loader_address: usize = self.boot_loader_address();
                let boot_loader_bytes: Vec<u8> = read(&boot_loader).unwrap();
                let pe: pe::File = boot_loader_bytes.as_slice().into();
                let section_name_pattern: Automaton = r"^\.\w+$".parse().unwrap();
                let mut section2address: HashMap<String, usize> = pe
                    .section_table()
                    .iter()
                    .filter_map(|section| {
                        let section_name: String = section.name();
                        let section_address: usize =
                            boot_loader_address + section.range_on_memory().start;
                        (!section_name_pattern.input(&section_name).is_empty())
                            .then_some((section_name, section_address))
                    })
                    .collect();
                let text_address: usize = section2address.remove(".text").unwrap();
                let section2address: Vec<String> = section2address
                    .into_iter()
                    .map(|(name, address)| format!("-s {} {:#x?}", name, address))
                    .collect();
                let section2address: String = section2address.join(" ");
                format!(
                    r#"
                        add-symbol-file {:?} {:#x?} {}
                        break 'boot::debug::breakpoint'
                        continue
                        set var *(&boot::debug::WAIT as *mut u8) = 0
                    "#,
                    boot_loader, text_address, section2address
                )
            }
            _ => String::default(),
        };
        write(
            gdbinit,
            format!(
                r#"
                    set architecture {}
                    file {:?}
                    target remote localhost:1234
                    {}
                    break 'boot::main'
                    continue
            "#,
                arch, boot_loader, option
            ),
        )
        .unwrap();
        run("RUST_GDB=gdb-multiarch rust-gdb");
    }

    fn arch(&self) -> &Arch {
        &self.0
    }

    fn boot(&self) -> Binary {
        let debuggee: Debuggee = self.into();
        debuggee.boot()
    }

    fn boot_loader_address(&self) -> usize {
        if let Some(boot_loader_address) = self.try_boot_loader_address() {
            boot_loader_address
        } else {
            sleep(Duration::from_secs(1));
            self.boot_loader_address()
        }
    }

    fn debug_log(&self) -> String {
        if let Ok(debug_log) = read_to_string(self.debug_log_path()) {
            debug_log
        } else {
            sleep(Duration::from_secs(1));
            self.debug_log()
        }
    }

    fn debug_log_path(&self) -> PathBuf {
        let debuggee: Debuggee = self.into();
        debuggee.debug_log()
    }

    fn new(arch: Arch) -> Self {
        Self(arch)
    }

    fn try_boot_loader_address(&self) -> Option<usize> {
        let automaton: Automaton =
            r"^Loading driver at 0x(?<address>[\da-fA-F]+) EntryPoint=0x[\da-fA-F]+ boot\.efi$"
                .parse()
                .unwrap();
        self.debug_log().lines().find_map(|line| {
            automaton.input(line).into_iter().next().and_then(|mat| {
                mat.captures()["address"]
                    .first()
                    .and_then(|address| usize::from_str_radix(address.into(), 16).ok())
            })
        })
    }
}

impl From<Args> for Debugger {
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
