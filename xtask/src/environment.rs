use crate::{docker, git, time};

pub fn attach() {
    let container: docker::Container = build();
    assert!(container.runs());
    container.attach();
}

pub fn privilege(gpg_key: &std::path::Path, ssh_key: &std::path::Path) {
    assert!(gpg_key.exists());
    assert!(gpg_key.is_dir());
    assert!(ssh_key.exists());
    assert!(ssh_key.is_file());
    let container: docker::Container = build();
    container.copy(gpg_key, &gpg_key_destination());
    container.copy(ssh_key, &ssh_key_destination());
    container.write(
        &ssh_config(),
        &format!(
            "Host {}\n\tHostName {}\n\tIdentityFile {}\n\tUser git",
            git::domain(),
            git::domain(),
            ssh_key_destination().to_str().unwrap()
        ),
    );
    [
        format!("git config --global user.name {}", git::developer()),
        format!("git config --global user.email {}", git::email()),
        "git config --global commit.gpgsign true".to_string(),
        format!(
            "git config --global user.signingkey {}",
            signing_key(gpg_key)
        ),
        format!(
            "git remote set-url origin git@{}:{}/{}.git",
            git::domain(),
            git::developer(),
            git::product()
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

pub fn remove() {
    if let Some(container) = container() {
        if container.runs() {
            container.stop();
        }
        container.remove();
        std::fs::remove_file(container_id_file()).unwrap();
    }
    if let Some(image) = image() {
        image.remove();
        std::fs::remove_file(image_id_file()).unwrap();
    }
}

fn build() -> docker::Container {
    let dockerfile: std::path::PathBuf = dockerfile();
    assert!(dockerfile.exists());
    let arguments: std::collections::BTreeMap<String, String> = [
        ("DOMAIN", git::domain()),
        ("DEVELOPER", git::developer()),
        ("PRODUCT", git::product()),
        ("BRANCH", git::branch()),
        ("TIMEZONE", time::zone()),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect();
    let image: docker::Image =
        image().unwrap_or_else(|| docker::Image::build(&dockerfile, &arguments, &image_id_file()));
    let container: docker::Container =
        container().unwrap_or_else(|| docker::Container::create(&image, &container_id_file()));
    if !container.runs() {
        container.start();
    }
    container
}

fn container() -> Option<docker::Container> {
    std::fs::read_to_string(container_id_file())
        .ok()
        .and_then(|id| id.as_str().try_into().ok())
}

fn container_id_file() -> std::path::PathBuf {
    std::path::PathBuf::from(".docker/container.id")
}

fn dockerfile() -> std::path::PathBuf {
    std::path::PathBuf::from(".docker/Dockerfile")
}

fn gpg_key_destination() -> std::path::PathBuf {
    let mut gpg_key_destination: std::path::PathBuf = home_directory();
    gpg_key_destination.push(".gnupg");
    gpg_key_destination
}

fn home_directory() -> std::path::PathBuf {
    build().home_directory()
}

fn image() -> Option<docker::Image> {
    std::fs::read_to_string(image_id_file())
        .ok()
        .and_then(|id| id.as_str().try_into().ok())
}

fn image_id_file() -> std::path::PathBuf {
    std::path::PathBuf::from(".docker/image.id")
}

fn signing_key(gpg_key: &std::path::Path) -> String {
    let mut signing_key: std::path::PathBuf = gpg_key.to_path_buf();
    signing_key.push("signingkey.txt");
    assert!(signing_key.exists());
    assert!(signing_key.is_file());
    std::fs::read_to_string(signing_key)
        .unwrap()
        .trim_end()
        .to_string()
}

fn ssh() -> std::path::PathBuf {
    let mut ssh: std::path::PathBuf = home_directory();
    ssh.push(".ssh");
    ssh
}

fn ssh_config() -> std::path::PathBuf {
    let mut ssh_config: std::path::PathBuf = ssh();
    ssh_config.push("config");
    ssh_config
}

fn ssh_key_destination() -> std::path::PathBuf {
    let mut ssh_key_destination: std::path::PathBuf = ssh();
    ssh_key_destination.push(git::domain());
    ssh_key_destination.push("key");
    ssh_key_destination
}
