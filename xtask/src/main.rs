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
    docker::image::build(image, dockerfile, arguments);
    docker::container::create(image, container);
    docker::container::start(container);
    docker::container::attach(container);
}
