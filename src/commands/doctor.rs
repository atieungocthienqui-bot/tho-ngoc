use colored::*;
use std::process::Command;
use sysinfo::System;

fn check_python() -> (String, bool) {
    match Command::new("python").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
            (ver, true)
        }
        _ => ("Not found".to_string(), false),
    }
}

fn check_piper() -> bool {
    match Command::new("python").args(["-m", "piper", "-h"]).output() {
        Ok(output) => output.status.success(),
        _ => false,
    }
}

pub async fn execute() {
    println!("\n      (\\_/)");
    println!("      ( •_•)     Project Thỏ Ngọc");
    println!("     / >🎙️      Checking your burrow...\n");

    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown");
    let ram_gb = sys.total_memory() / 1024 / 1024 / 1024;

    let (py_ver, py_ok) = check_python();
    let piper_ok = check_piper();

    println!("{:<15} {:<25} {}", "CPU", cpu_name, "✓".green());
    println!("{:<15} {} GB {:<19} {}", "RAM", ram_gb, "", "✓".green());
    
    if py_ok {
        println!("{:<15} {:<25} {}", "Python Bridge", py_ver, "✓".green());
    } else {
        println!("{:<15} {:<25} {}", "Python Bridge", "Not found", "✗".red());
    }

    println!("\nEngines");
    if piper_ok {
        println!("{:<30} {}", "Piper", "✓".green());
    } else {
        println!("{:<30} {}", "Piper", "Not installed (Run install.bat)".red());
    }
    println!("{:<30} {}", "Kokoro", "Not installed".bright_black());
    println!("{:<30} {}", "ZeroTTS", "Not installed".bright_black());

    println!("\nRecommended configuration:");
    println!("{:<20} Piper", "Vietnamese Fast");

    if ram_gb < 8 {
        println!("\nPotato Mode         {} 🥔 (RAM < 8GB detected)", "Auto-enabled".yellow());
    } else {
        println!("\nPotato Mode         {} 🥔", "Available".green());
    }
}
