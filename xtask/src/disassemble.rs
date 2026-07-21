use {crate::command::run, std::env::Args};

pub struct Command {
    package: Package,
    target: Target,
}

impl Command {
    pub fn run(self) {
        run(&format!(
            "llvm-objdump -d target/{}/debug/{}",
            self.target(),
            self.package()
        ));
    }

    fn package(&self) -> &str {
        let Self { package, target } = self;
        match (package, target) {
            (Package::Boot, Target::Aarch64) => "boot.efi",
            (Package::Boot, Target::Riscv64) => "boot",
            (Package::Boot, Target::X64) => "boot.efi",
        }
    }

    fn target(&self) -> &str {
        let Self { package, target } = self;
        match (package, target) {
            (Package::Boot, Target::Aarch64) => "aarch64-unknown-uefi",
            (Package::Boot, Target::Riscv64) => "riscv64gc-unknown-none-elf",
            (Package::Boot, Target::X64) => "x86_64-unknown-uefi",
        }
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let package: Package = args.next().unwrap().as_str().into();
        let target: Target = args.next().unwrap().as_str().into();
        Self { package, target }
    }
}

enum Package {
    Boot,
}

impl From<&str> for Package {
    fn from(package: &str) -> Self {
        match package {
            "boot" => Self::Boot,
            _ => unimplemented!(),
        }
    }
}

enum Target {
    Aarch64,
    Riscv64,
    X64,
}

impl From<&str> for Target {
    fn from(target: &str) -> Self {
        match target {
            "aarch64" => Self::Aarch64,
            "riscv64" => Self::Riscv64,
            "x64" => Self::X64,
            _ => unimplemented!(),
        }
    }
}
