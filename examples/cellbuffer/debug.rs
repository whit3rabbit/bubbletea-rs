use std::io::{self, Write};
use std::time::Duration;
use std::thread;

fn main() {
    // Test basic terminal output in Ghostty
    println!("Testing terminal output...");
    
    // Test clearing and cursor movement
    print!("\x1b[2J\x1b[H"); // Clear screen and move to home
    io::stdout().flush().unwrap();
    
    println!("If you see this, basic output works!");
    thread::sleep(Duration::from_secs(1));
    
    // Test drawing a simple pattern
    for y in 0..10 {
        for x in 0..20 {
            if (x + y) % 2 == 0 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    
    println!("\nPattern drawn. Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // Test cursor positioning
    print!("\x1b[2J\x1b[H"); // Clear again
    print!("\x1b[5;10H"); // Move to row 5, col 10
    print!("Cursor positioning test");
    io::stdout().flush().unwrap();
    
    thread::sleep(Duration::from_secs(2));
}