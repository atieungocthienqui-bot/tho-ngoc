use colored::*;

pub async fn execute() {
    println!("\n      (\\_/)");
    println!("      ( •_•)     Project Thỏ Ngọc");
    println!("     / >🎙️      Checking your burrow...\n");

    println!("{}\n", "[Not Implemented - Prototype Checks Only]".bright_black());

    println!("{:<15} {:<25} {}", "CPU", "Simulated", "✓".green());
    println!("{:<15} {:<25} {}", "RAM", "Simulated", "✓".green());
    println!("{:<15} {:<25} {}", "GPU", "Simulated", "✓".green());
    println!("{:<15} {:<25} {}", "Python Bridge", "Simulated", "✓".green());

    println!("\nEngines");
    println!("{:<30} {}", "Piper", "Simulated".green());
    println!("{:<30} {}", "Kokoro", "Simulated".green());
    println!("{:<30} {}", "ZeroTTS", "Not installed".bright_black());

    println!("\nRecommended configuration:");
    println!("{:<20} Piper", "Vietnamese Fast");
    println!("{:<20} Kokoro", "English Natural");

    println!("\nPotato Mode         {} 🥔", "Available".green());
}
