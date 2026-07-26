use {crate::product::Binary, std::env::Args};

pub struct Command(Binary);

impl Command {
    pub fn run(&self) {
        let Self(binary) = self;
        binary.disassemble();
    }
}

impl From<Args> for Command {
    fn from(args: Args) -> Self {
        let binary: Binary = args.into();
        Self(binary)
    }
}
