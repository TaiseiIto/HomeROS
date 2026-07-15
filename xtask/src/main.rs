mod command;
mod docker;
mod git;

fn main() {
    let name: String = git::name().to_lowercase();
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
    docker::image::build(image, dockerfile);
    docker::container::create(image, container);
    docker::container::start(container);
    docker::container::attach(container);
}
