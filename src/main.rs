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

    tokio::time::sleep(tokio::time::Duration::from_secs(200)).await;

    let result = Command::new("docker")
        .args(&["exec", "b-scylla", "nodetool", "status"])
        .output()
        .unwrap();

    println!("Scylla status: {:#?}", result);

    let output = Command::new("docker")
        .args(&[
            "exec",
            "b-elastic",
            "curl",
            "-X",
            "PUT",
            "http://localhost:9200/users",
            "-H",
            "Content-Type: application/json",
            "--data-binary",
            "@/src/setup.json",
        ])
        .output()
        .unwrap();

    println!("Elasticsearch result: {:#?}", output);
}
