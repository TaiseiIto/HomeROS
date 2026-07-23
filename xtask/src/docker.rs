use {
    crate::command::{get_stdout, give_stdin, run, test},
    std::{
        collections::BTreeMap,
        fs::{read_to_string, write},
        path::{Path, PathBuf},
    },
};

pub struct Container {
    id: String,
}

impl Container {
    pub fn attach(&self) {
        assert!(self.runs());
        run(&format!("docker attach {}", self.id));
    }

    pub fn create(image: &Image, id_file: &Path) -> Self {
        let id: String = get_stdout(&format!(
            "docker create --interactive --tty {} /bin/bash",
            image.id()
        ));
        write(id_file, &id).unwrap();
        Self { id }
    }

    pub fn execute(&self, command: &str) {
        assert!(self.runs());
        run(&format!("docker exec {} bash -cl '{}'", self.id, command));
    }

    pub fn execute_interactive(&self, command: &str) {
        assert!(self.runs());
        run(&format!(
            "docker exec --interactive {} bash -cl '{}'",
            self.id, command
        ));
    }

    pub fn export(&self, source: &Path, destination: &Path) {
        if let Some(destination_directory) = destination.parent() {
            run(&format!(
                "mkdir -p {}",
                destination_directory.to_str().unwrap()
            ));
        }
        run(&format!(
            "docker cp {}:{} {}",
            self.id,
            source.to_str().unwrap(),
            destination.to_str().unwrap()
        ));
    }

    pub fn get_stdout(&self, command: &str) -> String {
        assert!(self.runs());
        get_stdout(&format!("docker exec {} {}", self.id, command))
    }

    pub fn groups(&self) -> Vec<String> {
        assert!(self.runs());
        let mut groups: Vec<String> = self
            .get_stdout("groups")
            .split_whitespace()
            .map(|group| group.to_string())
            .collect();
        groups.sort();
        groups
    }

    pub fn home_directory(&self) -> PathBuf {
        assert!(self.runs());
        self.get_stdout("printenv HOME").into()
    }

    pub fn import(&self, source: &Path, destination: &Path) {
        assert!(self.runs());
        if let Some(destination_directory) = destination.parent() {
            self.make_directory(destination_directory);
        }
        run(&format!(
            "docker cp {} {}:{}",
            source.to_str().unwrap(),
            self.id,
            destination.to_str().unwrap()
        ));
    }

    pub fn remove(self) {
        assert!(!self.runs());
        run(&format!("docker rm {}", self.id));
    }

    pub fn runs(&self) -> bool {
        get_stdout(&format!(
            "docker inspect -f {{{{.State.Running}}}} {}",
            self.id
        )) == "true"
    }

    pub fn start(&self) {
        assert!(!self.runs());
        run(&format!("docker start {}", self.id));
    }

    pub fn stop(&self) {
        assert!(self.runs());
        run(&format!("docker stop {}", self.id));
    }

    pub fn user(&self) -> String {
        assert!(self.runs());
        self.get_stdout("whoami")
    }

    pub fn working_directory(&self) -> PathBuf {
        PathBuf::from(get_stdout(&format!(
            "docker inspect {} --format '{{{{.Config.WorkingDir}}}}'",
            self.id
        )))
    }

    pub fn write(&self, destination: &Path, data: &[u8]) {
        if !self.runs() {
            self.start();
        }
        give_stdin(
            &format!(
                "docker exec --interactive {} bash -c 'cat > {}'",
                self.id,
                destination.to_str().unwrap()
            ),
            data,
        );
    }

    fn make_directory(&self, directory: &Path) {
        self.execute(&format!("mkdir -p {}", directory.to_str().unwrap()));
    }
}

impl TryFrom<&str> for Container {
    type Error = ();

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        if test(&format!("docker inspect {}", id)) {
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
    pub fn build(dockerfile: &Path, arguments: &BTreeMap<String, String>, id_file: &Path) -> Self {
        let arguments: Vec<String> = arguments
            .iter()
            .map(|(key, value)| format!("--build-arg {}={}", key, value))
            .collect();
        let arguments: String = arguments.join(" ");
        run(&format!(
            "docker build --iidfile {} {} {}",
            id_file.to_str().unwrap(),
            dockerfile.parent().unwrap().to_str().unwrap(),
            arguments
        ));
        let id: String = read_to_string(id_file).unwrap();
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn remove(self) {
        run(&format!("docker image rm {}", self.id));
    }
}

impl TryFrom<&str> for Image {
    type Error = ();

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        if test(&format!("docker image inspect {}", id)) {
            let id: String = id.to_string();
            Ok(Self { id })
        } else {
            Err(())
        }
    }
}

pub fn is_installed() -> bool {
    test("which docker")
}
