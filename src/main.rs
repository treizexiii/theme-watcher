use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("dconf")
        .arg("watch")
        .arg("/org/gnome/desktop/interface/color-scheme")
        .stdout(Stdio::piped()) // Nous voulons lire la sortie
        .spawn()
        .expect("Failed to start dconf watch");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    println!("Watching color-scheme changes...");

    let mut last_theme = String::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // si la ligne contient un default ou prefer-dark
                if line.contains("default") || line.contains("prefer-dark") {
                    // si le theme est different du dernier theme
                    if line != last_theme {
                        last_theme = line.clone();
                        update_border_color(&line);
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let _ = child.wait().expect("Failed to wait on dconf watch");
}

fn update_border_color(theme: &str) {
    println!("Update Theme: {}", theme);
    let color = if theme.contains("dark") {
        "(1.0, 1.0, 1.0, 1.0)" // White for dark theme
    } else {
        "(0.0, 0.0, 0.0, 1.0)" // Black for light theme
    };

    // Appliquer la couleur via `dconf`
    let mut child = Command::new("dconf")
        .args(&[
            "write",
            "/org/gnome/shell/extensions/rounded-window-corners-reborn/border-color",
            color,
        ])
        .spawn()
        .expect("Failed to set border color");

    match child.wait() {
        Ok(status) if status.success() => {
            println!("Border color updated to {} for theme {}", color, theme)
        }
        Ok(_) => eprintln!("Failed to update border color for theme {}", theme),
        Err(_) => eprintln!("Failed to update border color for theme {}", theme),
    }
}
