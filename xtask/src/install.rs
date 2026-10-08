use {
    super::product::{Arch, Tree, Version},
    std::{env::Args, path::PathBuf},
};

#[derive(Debug)]
pub struct Command {
    virtual_disk: PathBuf,
}

impl Command {
    pub fn run(self) {
        match self.virtual_disk.extension().unwrap().to_str().unwrap() {
            "vhd" => {
                let source: PathBuf = Tree::new(Arch::X64, Version::Release).destination();
                let nbd: PathBuf = "/dev/nbd0".parse().unwrap();
                let mount_point: PathBuf = "mount_point".parse().unwrap();
                println!("source = {:#x?}", source);
                println!("nbd = {:#x?}", nbd);
                println!("mount_point = {:#x?}", mount_point);
            }
            "vmdk" => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        Self {
            virtual_disk: args.next().unwrap().into(),
        }
    }
}
