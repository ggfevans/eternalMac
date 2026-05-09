pub fn add(name: &str, local: &str, remote: &str) {
    println!("sync {name} {local} {remote}");
}

pub fn list() {
    println!("project");
}

pub fn status() {
    println!("sync healthy");
}
