use std::{env::Args, path::PathBuf};

#[derive(Debug)]
pub struct Command {
    #[allow(dead_code)]
    virtual_disk: PathBuf,
}

impl Command {
    pub fn run(self) {
        println!("{:#x?}", self);
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        Self {
            virtual_disk: args.next().unwrap().into(),
        }
    }
}
