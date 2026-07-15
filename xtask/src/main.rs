mod command;
mod docker;
mod git;
mod time;

fn main() {
    let name: String = git::product().to_lowercase();
    let container: &str = &name;
    let image: &str = &name;
    let dockerfile: &std::path::Path = std::path::Path::new(".docker/Dockerfile");
    if docker::container::runs(container) {
        docker::container::stop(container);
    }
    if docker::container::exists(container) {
        docker::container::remove(container);
    }
    if docker::image::exists(image) {
        docker::image::remove(image);
    }
    let mut arguments: std::collections::BTreeMap<String, String> =
        std::collections::BTreeMap::<String, String>::new();
    arguments.insert("DOMAIN".to_string(), git::domain());
    arguments.insert("DEVELOPER".to_string(), git::developer());
    arguments.insert("PRODUCT".to_string(), git::product());
    arguments.insert("TIMEZONE".to_string(), time::zone());
    docker::image::build(image, dockerfile, arguments);
    docker::container::create(image, container);
    docker::container::start(container);
    docker::container::attach(container);
}
