use {
    crate::{
        build::destination,
        docker::{Container, Image},
        git::{branch, developer, domain, email, product},
        run,
        time::zone,
    },
    std::{
        collections::BTreeMap,
        env::Args,
        fs::{read_to_string, remove_file},
        path::{Path, PathBuf},
    },
};

pub enum Command {
    Build,
    Delete,
    Privilege { gpg_key: PathBuf, ssh_key: PathBuf },
    Rebuild,
}

impl Command {
    pub fn run(self) {
        match self {
            Self::Build => attach(),
            Self::Delete => remove(),
            Self::Privilege { gpg_key, ssh_key } => privilege(&gpg_key, &ssh_key),
            Self::Rebuild => {
                remove();
                attach();
            }
        }
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        match args.next().as_deref() {
            None => Self::Build,
            Some("delete") => Self::Delete,
            Some("privilege") => {
                let mut gpg_key: Option<PathBuf> = None;
                let mut ssh_key: Option<PathBuf> = None;
                while let Some(arg) = args.next() {
                    match arg.as_str() {
                        "--gpg-key" => gpg_key = Some(args.next().unwrap().into()),
                        "--ssh-key" => ssh_key = Some(args.next().unwrap().into()),
                        arg => panic!("arg = {}", arg),
                    }
                }
                let gpg_key: PathBuf = gpg_key.unwrap();
                let ssh_key: PathBuf = ssh_key.unwrap();
                Self::Privilege { gpg_key, ssh_key }
            }
            Some("rebuild") => Self::Rebuild,
            Some(arg) => panic!("arg = {}", arg),
        }
    }
}

pub fn build_in_container() {
    let container: Container = build();
    let source: PathBuf = container.working_directory().join(destination());
    let destination: PathBuf = destination();
    assert!(container.runs());
    container.execute("cargo xtask build");
    container.export(&source, &destination);
}

pub fn run_in_container(command: run::Command) {
    let container: Container = build();
    assert!(container.runs());
    container.execute_interactive(&format!("cargo xtask run {}", command));
}

fn attach() {
    let container: Container = build();
    assert!(container.runs());
    container.attach();
}

fn privilege(gpg_key: &Path, ssh_key: &Path) {
    assert!(gpg_key.exists());
    assert!(gpg_key.is_dir());
    assert!(ssh_key.exists());
    assert!(ssh_key.is_file());
    let container: Container = build();
    container.import(gpg_key, &gpg_key_destination());
    container.import(ssh_key, &ssh_key_destination());
    container.write(
        &ssh_config(),
        &format!(
            "Host {}\n\tHostName {}\n\tIdentityFile {}\n\tUser git",
            domain(),
            domain(),
            ssh_key_destination().to_str().unwrap()
        )
        .into_bytes(),
    );
    [
        format!("git config --global user.name {}", developer()),
        format!("git config --global user.email {}", email()),
        "git config --global commit.gpgsign true".to_string(),
        format!(
            "git config --global user.signingkey {}",
            signing_key(gpg_key)
        ),
        format!(
            "git remote set-url origin git@{}:{}/{}.git",
            domain(),
            developer(),
            product()
        ),
        format!(
            "chown -R {}:{} {}",
            container.user(),
            container.groups().pop().unwrap(),
            gpg_key_destination().to_str().unwrap()
        ),
        format!("chmod -R 600 {}", gpg_key_destination().to_str().unwrap()),
        format!(
            "chown -R {}:{} {}",
            container.user(),
            container.groups().pop().unwrap(),
            ssh().to_str().unwrap()
        ),
        format!("chmod -R 600 {}", ssh().to_str().unwrap()),
    ]
    .into_iter()
    .for_each(|command| {
        container.execute(&command);
    });
}

fn remove() {
    if let Some(container) = container() {
        if container.runs() {
            container.stop();
        }
        container.remove();
        remove_file(container_id_file()).unwrap();
    }
    if let Some(image) = image() {
        image.remove();
        remove_file(image_id_file()).unwrap();
    }
}

fn build() -> Container {
    let dockerfile: PathBuf = dockerfile();
    assert!(dockerfile.exists());
    let arguments: BTreeMap<String, String> = [
        ("DOMAIN", domain()),
        ("DEVELOPER", developer()),
        ("PRODUCT", product()),
        ("BRANCH", branch()),
        ("TIMEZONE", zone()),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect();
    let image: Image =
        image().unwrap_or_else(|| Image::build(&dockerfile, &arguments, &image_id_file()));
    let container: Container =
        container().unwrap_or_else(|| Container::create(&image, &container_id_file()));
    if !container.runs() {
        container.start();
    }
    container
}

fn container() -> Option<Container> {
    read_to_string(container_id_file())
        .ok()
        .and_then(|id| id.as_str().try_into().ok())
}

fn container_id_file() -> PathBuf {
    PathBuf::from(".docker/container.id")
}

fn dockerfile() -> PathBuf {
    PathBuf::from(".docker/Dockerfile")
}

fn gpg_key_destination() -> PathBuf {
    let mut gpg_key_destination: PathBuf = home_directory();
    gpg_key_destination.push(".gnupg");
    gpg_key_destination
}

fn home_directory() -> PathBuf {
    build().home_directory()
}

fn image() -> Option<Image> {
    read_to_string(image_id_file())
        .ok()
        .and_then(|id| id.as_str().try_into().ok())
}

fn image_id_file() -> PathBuf {
    PathBuf::from(".docker/image.id")
}

fn signing_key(gpg_key: &Path) -> String {
    let mut signing_key: PathBuf = gpg_key.to_path_buf();
    signing_key.push("signingkey.txt");
    assert!(signing_key.exists());
    assert!(signing_key.is_file());
    read_to_string(signing_key).unwrap().trim_end().to_string()
}

fn ssh() -> PathBuf {
    let mut ssh: PathBuf = home_directory();
    ssh.push(".ssh");
    ssh
}

fn ssh_config() -> PathBuf {
    let mut ssh_config: PathBuf = ssh();
    ssh_config.push("config");
    ssh_config
}

fn ssh_key_destination() -> PathBuf {
    let mut ssh_key_destination: PathBuf = ssh();
    ssh_key_destination.push(domain());
    ssh_key_destination.push("key");
    ssh_key_destination
}
