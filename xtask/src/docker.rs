use crate::command;

pub struct Container {
    id: String,
}

impl Container {
    pub fn attach(&self) {
        assert!(self.runs());
        command::run(&format!("docker attach {}", self.id));
    }

    pub fn copy(&self, source: &std::path::Path, destination: &std::path::Path) {
        assert!(self.runs());
        if let Some(destination_directory) = destination.parent() {
            self.make_directory(destination_directory);
        }
        command::run(&format!(
            "docker cp {} {}:{}",
            source.to_str().unwrap(),
            self.id,
            destination.to_str().unwrap()
        ));
    }

    pub fn create(image: &Image, id_file: &std::path::Path) -> Self {
        let id: String = command::get_stdout(&format!(
            "docker create --interactive --tty {} /bin/bash",
            image.id()
        ));
        std::fs::write(id_file, &id).unwrap();
        Self { id }
    }

    pub fn execute(&self, command: &str) -> String {
        assert!(self.runs());
        command::get_stdout(&format!("docker exec {} {}", self.id, command))
    }

    pub fn groups(&self) -> Vec<String> {
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
        assert!(self.runs());
        self.execute("printenv HOME").into()
    }

    pub fn remove(self) {
        assert!(!self.runs());
        command::run(&format!("docker rm {}", self.id));
    }

    pub fn runs(&self) -> bool {
        command::get_stdout(&format!(
            "docker inspect -f {{{{.State.Running}}}} {}",
            self.id
        )) == "true"
    }

    pub fn start(&self) {
        assert!(!self.runs());
        command::run(&format!("docker start {}", self.id));
    }

    pub fn stop(&self) {
        assert!(self.runs());
        command::run(&format!("docker stop {}", self.id));
    }

    pub fn user(&self) -> String {
        assert!(self.runs());
        self.execute("whoami")
    }

    pub fn write(&self, destination: &std::path::Path, data: &[u8]) {
        assert!(!self.runs());
        command::give_stdin(
            &format!(
                "docker exec {} 'cat > {}'",
                self.id,
                destination.to_str().unwrap()
            ),
            data,
        );
    }

    fn make_directory(&self, directory: &std::path::Path) {
        self.execute(&format!("mkdir {}", directory.to_str().unwrap()));
    }
}

impl TryFrom<&str> for Container {
    type Error = ();

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        if command::test(&format!("docker inspect {}", id)) {
            let id: String = id.to_string();
            Ok(Self { id })
        } else {
            Err(())
        }
    }
}

pub struct Image {
    id: String,
}

impl Image {
    pub fn build(
        dockerfile: &std::path::Path,
        arguments: &std::collections::BTreeMap<String, String>,
        id_file: &std::path::Path,
    ) -> Self {
        let arguments: Vec<String> = arguments
            .iter()
            .map(|(key, value)| format!("--build-arg {}={}", key, value))
            .collect();
        let arguments: String = arguments.join(" ");
        command::run(&format!(
            "docker build --iidfile {} {} {}",
            id_file.to_str().unwrap(),
            dockerfile.parent().unwrap().to_str().unwrap(),
            arguments
        ));
        let id: String = std::fs::read_to_string(id_file).unwrap();
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn remove(self) {
        command::run(&format!("docker image rm {}", self.id));
    }
}

impl TryFrom<&str> for Image {
    type Error = ();

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        if command::test(&format!("docker image inspect {}", id)) {
            let id: String = id.to_string();
            Ok(Self { id })
        } else {
            Err(())
        }
    }
}
