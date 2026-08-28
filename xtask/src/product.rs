use {
    crate::{command::run, firmware::tfa},
    std::{
        env::Args,
        fmt::{Display, Formatter, Result},
        path::PathBuf,
    },
};

pub fn build() {
    for binary in Binary::domain().into_iter() {
        binary.build();
    }
}

pub fn destination() -> PathBuf {
    PathBuf::from("target")
}

pub fn lint() {
    for binary in Binary::domain().into_iter() {
        binary.lint();
    }
}

pub struct Binary {
    package: Package,
    tree: Tree,
}

impl Binary {
    pub fn disassemble(&self) {
        run(&format!(
            "llvm-objdump -d {}",
            self.source().to_str().unwrap()
        ));
    }

    fn build(&self) {
        run(&format!(
            "{} cargo build --package {} {} --target {}",
            self.vars(),
            self.package,
            self.tree.version.argument(),
            self.target(),
        ));
        run(&format!(
            "mkdir -p {}",
            self.destination().parent().unwrap().to_str().unwrap()
        ));
        let Self {
            package,
            tree: Tree { arch, version: _ },
        } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => {
                run(&format!(
                    "llvm-objcopy -O binary {} {}",
                    self.source().to_str().unwrap(),
                    tfa::bl33().to_str().unwrap()
                ));
                run(&format!(
                    "make -C {} PLAT=qemu DEBUG=1 BL33={} qemu_fw.rom",
                    tfa::top().to_str().unwrap(),
                    tfa::bl33().to_str().unwrap()
                ));
                run(&format!(
                    "cp {} {}",
                    tfa::rom().to_str().unwrap(),
                    self.destination().to_str().unwrap()
                ));
            }
            _ => {
                run(&format!(
                    "cp {} {}",
                    self.source().to_str().unwrap(),
                    self.destination().to_str().unwrap()
                ));
            }
        }
    }

    fn destination(&self) -> PathBuf {
        let Self {
            package,
            tree: tree @ Tree { arch, version: _ },
        } = self;
        let destination: PathBuf = tree.destination();
        let disk_relative_path: &str = match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "qemu_fw.rom",
            (Arch::RiscV64, Package::Boot) => "boot.elf",
            (Arch::X64, Package::Boot) => "EFI/BOOT/BOOTX64.EFI",
        };
        destination.as_path().join(disk_relative_path)
    }

    fn domain() -> Vec<Self> {
        Package::domain()
            .into_iter()
            .flat_map(|package| {
                Tree::domain().into_iter().map(move |tree| {
                    let package: Package = package.clone();
                    Self { package, tree }
                })
            })
            .collect()
    }

    fn lint(&self) {
        run(&format!(
            "{} cargo clippy --package {} --target {}",
            self.vars(),
            self.package,
            self.target()
        ));
    }

    fn name(&self) -> &str {
        let Self {
            package,
            tree: Tree { arch, version: _ },
        } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "boot",
            (Arch::RiscV64, Package::Boot) => "boot",
            (Arch::X64, Package::Boot) => "boot.efi",
        }
    }

    fn new(arch: Arch, package: Package, version: Version) -> Self {
        let tree: Tree = Tree { arch, version };
        Self { package, tree }
    }

    fn source(&self) -> PathBuf {
        PathBuf::from(&format!(
            "target/{}/{}/{}",
            self.target(),
            self.tree.version,
            self.name()
        ))
    }

    fn target(&self) -> &str {
        let Self {
            package,
            tree: Tree { arch, version: _ },
        } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "aarch64-unknown-none-softfloat",
            (Arch::RiscV64, Package::Boot) => "riscv64gc-unknown-none-elf",
            (Arch::X64, Package::Boot) => "x86_64-unknown-uefi",
        }
    }

    fn vars(&self) -> &str {
        let Self {
            package,
            tree: Tree { arch, version: _ },
        } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "RUSTFLAGS=\"-C link-arg=boot/link/aarch64.ld\"",
            (Arch::RiscV64, Package::Boot) => "RUSTFLAGS=\"-C link-arg=boot/link/riscv64.ld\"",
            (Arch::X64, Package::Boot) => "",
        }
    }
}

impl From<Args> for Binary {
    fn from(mut args: Args) -> Self {
        let mut arch: Option<Arch> = None;
        let mut package: Option<Package> = None;
        let mut version: Option<Version> = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--arch" => arch = Some(args.next().unwrap().as_str().into()),
                "--package" => package = Some(args.next().unwrap().as_str().into()),
                "--version" => version = Some(args.next().unwrap().as_str().into()),
                arg => unreachable!("arg = {}", arg),
            }
        }
        Self::new(arch.unwrap(), package.unwrap(), version.unwrap())
    }
}

#[derive(Clone)]
pub enum Arch {
    Aarch64,
    RiscV64,
    X64,
}

impl Arch {
    pub fn boot_destination(&self, version: &Version) -> PathBuf {
        Binary::new(self.clone(), Package::Boot, version.clone()).destination()
    }

    pub fn domain() -> Vec<Self> {
        [Self::Aarch64, Self::RiscV64, Self::X64]
            .into_iter()
            .collect()
    }
}

impl Display for Arch {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let arch: &str = self.into();
        write!(formatter, "{}", arch)
    }
}

impl From<&Arch> for &str {
    fn from(arch: &Arch) -> Self {
        match arch {
            Arch::Aarch64 => "aarch64",
            Arch::RiscV64 => "riscv64",
            Arch::X64 => "x64",
        }
    }
}

impl From<&str> for Arch {
    fn from(arch: &str) -> Self {
        match arch {
            "aarch64" => Self::Aarch64,
            "riscv64" => Self::RiscV64,
            "x64" => Self::X64,
            arch => unimplemented!("arch = {}", arch),
        }
    }
}

pub struct Tree {
    arch: Arch,
    version: Version,
}

impl Tree {
    pub fn domain() -> Vec<Self> {
        Arch::domain()
            .into_iter()
            .flat_map(|arch| {
                Version::domain().into_iter().map(move |version| {
                    let arch: Arch = arch.clone();
                    Self { arch, version }
                })
            })
            .collect()
    }

    pub fn destination(&self) -> PathBuf {
        let Self { arch, version } = self;
        let mut destination: PathBuf = version.destination();
        destination.push(format!("{}", arch));
        destination
    }

    pub fn new(arch: Arch, version: Version) -> Self {
        Self { arch, version }
    }
}

#[derive(Clone)]
pub enum Version {
    Debug,
    Release,
}

impl Version {
    pub fn domain() -> Vec<Self> {
        [Self::Debug, Self::Release].into_iter().collect()
    }

    fn argument(&self) -> &str {
        match self {
            Self::Debug => "",
            Self::Release => "--release",
        }
    }

    fn destination(&self) -> PathBuf {
        let mut destination: PathBuf = destination();
        destination.push(format!("{}", self));
        destination
    }
}

impl Display for Version {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let version: &str = self.into();
        write!(formatter, "{}", version)
    }
}

impl From<&Version> for &str {
    fn from(version: &Version) -> Self {
        match version {
            Version::Debug => "debug",
            Version::Release => "release",
        }
    }
}

impl From<&str> for Version {
    fn from(version: &str) -> Self {
        match version {
            "debug" => Self::Debug,
            "release" => Self::Release,
            version => unimplemented!("version = {}", version),
        }
    }
}

#[derive(Clone)]
enum Package {
    Boot,
}

impl Package {
    fn domain() -> Vec<Self> {
        [Self::Boot].into_iter().collect()
    }
}

impl Display for Package {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let package: &str = self.into();
        write!(formatter, "{}", package)
    }
}

impl From<&Package> for &str {
    fn from(package: &Package) -> Self {
        match package {
            Package::Boot => "boot",
        }
    }
}

impl From<&str> for Package {
    fn from(package: &str) -> Self {
        match package {
            "boot" => Self::Boot,
            package => unreachable!("package = {}", package),
        }
    }
}
