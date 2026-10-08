use {
    super::{
        command,
        product::{Arch, Tree, Version},
    },
    std::{env::Args, path::PathBuf, thread::sleep, time::Duration},
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
                command::run("sudo modprobe nbd max_part=16");
                command::run(&format!(
                    "sudo qemu-nbd --format=vpc --connect={:#x?} {:#x?}",
                    nbd, self.virtual_disk
                ));
                while command::get_stdout(&format!("sudo blockdev --getsize64 {:#x?}", nbd))
                    .as_str()
                    .parse()
                    == Ok(0usize)
                {
                    sleep(Duration::from_millis(100));
                }
                command::run(&format!("sudo mkfs.vfat -v -c -F 32 {:#x?}", nbd));
                command::run(&format!("mkdir {:#x?}", mount_point));
                command::run(&format!("sudo mount {:#x?} {:#x?}", nbd, mount_point));
                command::run(&format!("sudo cp -r {:#x?}/* {:#x?}", source, mount_point));
                command::run(&format!("sudo umount {:#x?}", mount_point));
                command::run(&format!("sudo rm -rf {:#x?}", mount_point));
                command::run(&format!("sudo qemu-nbd --disconnect {:#x?}", nbd));
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
