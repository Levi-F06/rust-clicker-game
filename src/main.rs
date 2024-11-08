fn clear_screen() {
    std::process::Command::new("tput")
        .arg("clear")
        .status()
        .expect("Failed to clear screen");
}

fn click(mut player_clicks: u32) {
    loop {
        clear_screen();

        let mut command = String::new();

        println!("____________________");
        println!("\n\n -> Coins: {player_clicks}");
        println!("\n\nCommands:");
        println!("Enter key -> click");
        println!("S -> Shop");
        println!("E -> Exit");
        println!("R -> Reset");
        println!("\n\n____________________");

        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command == "\n" {
            player_clicks = player_clicks + 1;
            continue;
        }

        command = command.trim().to_lowercase();

        if command == "e" || command == "exit" {
            break;
        }
        if command == "s" || command == "shop" {
            break shop();
        }
        if command == "r" || command == "reset" {
            break click(0);
        }
    }
}

fn shop() {
    println!("WIP");
}

fn main() {
    click(0);
}
