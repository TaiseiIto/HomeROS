mod docker;
mod git;

fn main() {
    let name: String = git::name().to_lowercase();
    let container: &str = &name;
    let image: &str = &name;
    let dockerfile: &std::path::Path = std::path::Path::new(".docker/Dockerfile");
    docker::container::stop(container);
    docker::container::remove(container);
    docker::image::remove(image);
    docker::image::build(image, dockerfile);
    docker::container::create(image, container);
}
