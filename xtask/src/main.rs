mod docker;
mod git;

fn main() {
    let name: String = git::name().to_lowercase();
    let container: &str = &name;
    let image: &str = &name;
    let dockerfile: &std::path::Path = std::path::Path::new(".docker/Dockerfile");
    docker::stop_container(container);
    docker::remove_container(container);
    docker::remove_image(image);
    docker::build(image, dockerfile);
    docker::create(image, container);
}
