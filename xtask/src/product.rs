use {
    crate::{command::run, git::product},
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
    arch: Arch,
    package: Package,
}

impl Binary {
    pub fn disassemble(&self) {
        run(&format!(
            "llvm-objdump -d target/{}/debug/{}",
            self.target(),
            self.package
        ));
    }

    fn build(&self) {
        run(&format!(
            "{} cargo build --package {} --target {}",
            self.vars(),
            self.package,
            self.target()
        ));
        run(&format!(
            "mkdir -p {}",
            self.destination().parent().unwrap().to_str().unwrap()
        ));
        run(&format!(
            "cp {} {}",
            self.source().to_str().unwrap(),
            self.destination().to_str().unwrap()
        ));
    }

    fn destination(&self) -> PathBuf {
        let mut destination: PathBuf = destination();
        destination.push(product());
        destination.push(format!("{}", self.arch));
        let Self { arch, package } = self;
        let disk_relative_path: &str = match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "efi/boot/bootaa64.efi",
            (Arch::RiscV64, Package::Boot) => "boot.elf",
            (Arch::X64, Package::Boot) => "efi/boot/bootx64.efi",
        };
        destination.as_path().join(disk_relative_path)
    }

    fn domain() -> Vec<Self> {
        Arch::domain()
            .into_iter()
            .flat_map(|arch| {
                Package::domain().into_iter().map(move |package| {
                    let arch: Arch = arch.clone();
                    Self { arch, package }
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
        let Self { arch, package } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "boot.efi",
            (Arch::RiscV64, Package::Boot) => "boot",
            (Arch::X64, Package::Boot) => "boot.efi",
        }
    }

    fn source(&self) -> PathBuf {
        PathBuf::from(&format!("target/{}/debug/{}", self.target(), self.name()))
    }

    fn target(&self) -> &str {
        let Self { arch, package } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "aarch64-unknown-uefi",
            (Arch::RiscV64, Package::Boot) => "riscv64gc-unknown-none-elf",
            (Arch::X64, Package::Boot) => "x86_64-unknown-uefi",
        }
    }

    fn vars(&self) -> &str {
        let Self { arch, package } = self;
        match (arch, package) {
            (Arch::Aarch64, Package::Boot) => "",
            (Arch::RiscV64, Package::Boot) => {
                "RUSTFLAGS=\"-C link-arg=boot/firmware/open_sbi/link.ld\""
            }
            (Arch::X64, Package::Boot) => "",
        }
    }
}

impl From<Args> for Binary {
    fn from(mut args: Args) -> Self {
        let mut arch: Option<Arch> = None;
        let mut package: Option<Package> = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--arch" => arch = Some(args.next().unwrap().as_str().into()),
                "--package" => package = Some(args.next().unwrap().as_str().into()),
                arg => unreachable!("arg = {}", arg),
            }
        }
        let arch: Arch = arch.unwrap();
        let package: Package = package.unwrap();
        Self { arch, package }
    }
}

#[derive(Clone)]
enum Arch {
    Aarch64,
    RiscV64,
    X64,
}

impl Arch {
    fn domain() -> Vec<Self> {
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
