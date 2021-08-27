use std::process::Command;
use std::env::current_dir;

#[tokio::main]
async fn main() {
    let docker_compose_dir = current_dir().unwrap().join("src");

    assert!(docker_compose_dir.exists(), "{:#?}", docker_compose_dir);

    let result = Command::new("docker-compose")
        .current_dir(&docker_compose_dir)
        .args(&["up", "-d"])
        .status()
        .unwrap();

    assert!(result.success(), "{:#?}", result);

    tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;

    let result = Command::new("docker")
        .args(&["exec", "b-scylla", "nodetool", "status"])
        .status()
        .unwrap();

    panic!("{:#?}", result);
}
