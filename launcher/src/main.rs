use std::{env, process::Command};

fn main() {
    let client = match env::args().nth(1).as_deref() {
        Some("cli") => "cli",
        _ => "gui",
    };

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args([
                "/C",
                "start",
                "cmd",
                "/K",
                "cargo run -p server .\\server\\config.yaml",
            ])
            .spawn()
            .expect("Cannot start the server");
    }

    #[cfg(unix)]
    {
        Command::new("gnome-terminal")
            .args([
                "--",
                "sh",
                "-c",
                "cargo run -p server ./server/config.yaml; exec bash"
            ])
            .spawn()
            .expect("Cannot start the server (Vérifie que gnome-terminal est installé)");
    }

    let start_client = Command::new("cargo")
        .args(["run", "-p", client])
        .status()
        .expect("Cannot start the client");

    std::process::exit(start_client.code().unwrap_or(1));
}
