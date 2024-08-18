use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=client/");

    let status = Command::new("npm")
        .args(["run", "build"])
        .current_dir("./client")
        .status()
        .expect("Failed to build Curoku client app");

    if !status.success() {
        println!("cargo::warning=Failed to build the client app. Investigate the problem or run the build manually");
    }
}
