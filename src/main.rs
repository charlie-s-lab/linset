use std::process::Command;

fn detect_linux_distribution() -> Option<String> {
    let output = Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let content = String::from_utf8_lossy(&output.stdout);
        for line in content.lines() {
            if line.starts_with("ID=") {
                return Some(line.replace("ID=", "").replace("\"", ""));
            }
        }
    }
    None
}

fn execute_action(distribution: &str) {
    match distribution {
        "Ubuntu" => println!("Aktion für Ubuntu ausführen"),
        "Fedora" => println!("Aktion für Fedora ausführen"),
        "Debian" => println!("Aktion für Debian ausführen"),
        "Arch" => println!("Aktion für Arch Linux ausführen"),
        "Manjaro" => println!("Aktion für Manjaro ausführen"),
        "openSUSE" => println!("Aktion für openSUSE ausführen"),
        "Mint" => println!("Aktion für Linux Mint ausführen"),
        "Gentoo" => println!("Aktion für Gentoo ausführen"),
        "LMDE" => println!("Aktion für LMDE ausführen"),
        _ => println!("Unbekannte Distribution: {}", distribution),
    }
}

fn main() {
    if let Some(distribution) = detect_linux_distribution() {
        println!("Linux Distribution: {:?}", distribution);
        execute_action(&distribution);
    } else {
        println!("Konnte die Linux-Distribution nicht erkennen");
    }
}
