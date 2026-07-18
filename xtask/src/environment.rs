use crate::{docker, git, time};
use indoc::indoc;

pub fn attach() {
    build();
    let container: std::path::PathBuf = container();
    assert!(docker::container::exists(&container));
    assert!(docker::container::runs(&container));
    docker::container::attach(&container);
}

pub fn privilege(gpg_key: &std::path::Path, ssh_key: &std::path::Path) {
    assert!(gpg_key.exists());
    assert!(gpg_key.is_dir());
    assert!(ssh_key.exists());
    assert!(ssh_key.is_file());
    build();
    let container: std::path::PathBuf = container();
    docker::container::copy(gpg_key, &container, &gpg_key_destination());
    docker::container::copy(ssh_key, &container, &ssh_key_destination());
    docker::container::write(
        &container,
        &ssh_config(),
        &format!(
            indoc! {r#"
                Host {}
                    HostName {}
                    IdentityFile {}
                    User git
        "#},
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
            docker::container::user(&container),
            docker::container::groups(&container).pop().unwrap(),
            gpg_key_destination().to_str().unwrap()
        ),
        format!("chmod -R 600 {}", gpg_key_destination().to_str().unwrap()),
        format!(
            "chown -R {}:{} {}",
            docker::container::user(&container),
            docker::container::groups(&container).pop().unwrap(),
            ssh().to_str().unwrap()
        ),
        format!("chmod -R 600 {}", ssh().to_str().unwrap()),
    ]
    .into_iter()
    .for_each(|command| {
        docker::container::execute(&container, &command);
    });
}

pub fn remove() {
    let image: std::path::PathBuf = image();
    let container: std::path::PathBuf = container();
    if docker::container::runs(&container) {
        docker::container::stop(&container);
    }
    if docker::container::exists(&container) {
        docker::container::remove(&container);
    }
    if docker::image::exists(&image) {
        docker::image::remove(&image);
    }
}

fn build() {
    let image: std::path::PathBuf = image();
    let container: std::path::PathBuf = container();
    let dockerfile: std::path::PathBuf = dockerfile();
    assert!(dockerfile.exists());
    let arguments: std::collections::BTreeMap<String, String> = [
        ("CACHE_BUSTER", time::unix()),
        ("DOMAIN", git::domain()),
        ("DEVELOPER", git::developer()),
        ("PRODUCT", git::product()),
        ("BRANCH", git::branch()),
        ("TIMEZONE", time::zone()),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect();
    if !docker::image::exists(&image) {
        docker::image::build(&image, &dockerfile, arguments);
    }
    if !docker::container::exists(&container) {
        docker::container::create(&image, &container);
    }
    if !docker::container::runs(&container) {
        docker::container::start(&container);
    }
}

fn container() -> std::path::PathBuf {
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
    build();
    docker::container::home_directory(&container())
}

fn image() -> std::path::PathBuf {
    std::path::PathBuf::from(".docker/image.id")
}

fn signing_key(gpg_key: &std::path::Path) -> String {
    let mut signing_key: std::path::PathBuf = gpg_key.to_path_buf();
    signing_key.push("signingkey.txt");
    assert!(signing_key.exists());
    assert!(signing_key.is_file());
    assert!(!signing_key.is_empty());
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
