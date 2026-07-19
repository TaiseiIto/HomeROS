use {crate::command, std::io::Write};

pub struct Container {
    id_file: std::path::PathBuf,
}

impl Container {
    pub fn attach(&self) {
        assert!(self.exists());
        assert!(self.runs());
        command::run(&format!("docker attach {}", self.read_id()));
    }

    pub fn copy(&self, source: &std::path::Path, destination: &std::path::Path) {
        assert!(self.exists());
        assert!(self.runs());
        if let Some(destination_directory) = destination.parent() {
            self.make_directory(destination_directory);
        }
        command::run(&format!(
            "docker cp {} {}:{}",
            source.to_str().unwrap(),
            self.read_id(),
            destination.to_str().unwrap()
        ));
    }

    pub fn create(&self, image: &Image) {
        assert!(!self.exists());
        let id: String = command::get_stdout(&format!(
            "docker create --interactive --tty {} /bin/bash",
            image.read_id()
        ));
        std::fs::write(&self.id_file, id).unwrap();
    }

    pub fn execute(&self, command: &str) -> String {
        assert!(self.exists());
        assert!(self.runs());
        command::get_stdout(&format!("docker exec {} {}", self.read_id(), command))
    }

    pub fn exists(&self) -> bool {
        self.id_file.exists() && self.id_file.is_file() && {
            let my_id: String = self.read_id();
            command::test(&format!("docker inspect {}", my_id))
        }
    }

    pub fn groups(&self) -> Vec<String> {
        assert!(self.exists());
        assert!(self.runs());
        let mut groups: Vec<String> = self
            .execute("groups")
            .split_whitespace()
            .map(|group| group.to_string())
            .collect();
        groups.sort();
        groups
    }

    pub fn home_directory(&self) -> std::path::PathBuf {
        assert!(self.exists());
        assert!(self.runs());
        self.execute("printenv HOME").into()
    }

    pub fn remove(&self) {
        assert!(self.exists());
        assert!(!self.runs());
        command::run(&format!("docker rm {}", self.read_id()));
    }

    pub fn runs(&self) -> bool {
        self.exists()
            && command::get_stdout(&format!(
                "docker inspect -f {{{{.State.Running}}}} {}",
                self.read_id()
            )) == "true"
    }

    pub fn start(&self) {
        assert!(self.exists());
        assert!(!self.runs());
        command::run(&format!("docker start {}", self.read_id()));
    }

    pub fn stop(&self) {
        assert!(self.exists());
        assert!(self.runs());
        command::run(&format!("docker stop {}", self.read_id()));
    }

    pub fn user(&self) -> String {
        assert!(self.exists());
        assert!(self.runs());
        self.execute("whoami")
    }

    pub fn write(&self, destination: &std::path::Path, data: &str) {
        let mut temporary: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(temporary, "{}", data).unwrap();
        self.copy(temporary.path(), destination);
    }

    fn make_directory(&self, directory: &std::path::Path) {
        self.execute(&format!("mkdir {}", directory.to_str().unwrap()));
    }

    fn read_id(&self) -> String {
        let Self { id_file } = self;
        assert!(id_file.exists());
        assert!(id_file.is_file());
        std::fs::read_to_string(id_file).unwrap()
    }
}

impl From<&str> for Container {
    fn from(id_file: &str) -> Self {
        let id_file: std::path::PathBuf = std::path::PathBuf::from(id_file);
        Self { id_file }
    }
}

pub struct Image {
    id_file: std::path::PathBuf,
}

impl Image {
    pub fn build(
        &self,
        dockerfile: &std::path::Path,
        arguments: std::collections::BTreeMap<String, String>,
    ) {
        assert!(!self.exists());
        let arguments: Vec<String> = arguments
            .iter()
            .map(|(key, value)| format!("--build-arg {}={}", key, value))
            .collect();
        let arguments: String = arguments.join(" ");
        command::run(&format!(
            "docker build --iidfile {} {} {}",
            self.id_file.to_str().unwrap(),
            dockerfile.parent().unwrap().to_str().unwrap(),
            arguments
        ));
    }

    pub fn exists(&self) -> bool {
        self.id_file.exists() && self.id_file.is_file() && {
            let my_id: String = self.read_id();
            command::test(&format!("docker image inspect {}", my_id))
        }
    }

    pub fn read_id(&self) -> String {
        let Self { id_file } = self;
        assert!(id_file.exists());
        assert!(id_file.is_file());
        std::fs::read_to_string(id_file).unwrap()
    }

    pub fn remove(&self) {
        assert!(self.exists());
        command::run(&format!("docker image rm {}", self.read_id()));
    }
}

impl From<&str> for Image {
    fn from(id_file: &str) -> Self {
        let id_file: std::path::PathBuf = std::path::PathBuf::from(id_file);
        Self { id_file }
    }
}
