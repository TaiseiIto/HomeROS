use crate::command;

pub mod container;

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
