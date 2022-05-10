mod robot;
mod task;
mod bot_type;
mod bot_manager;

use crate::bot_manager::BotManager;
use crate::robot::Robot;
use crate::bot_type::BotType;

use std::io::{stdout, stdin, Write};
use::std::process::exit;

fn main() {
    let mut manager = BotManager::new();
    let menu = "\nMain Menu\n---------\n0) Quit\n1) Create a Bot\n2) View Bots\n";

    loop {
        println!("{}", menu);
        let mut choice = prompt_choice("Pick a menu option");

        while !(choice >= 0 && choice < 3) {
            println!("\n* Pick a valid menu option!\n");
            choice = prompt_choice("Pick a menu option");
        }

        match choice {
            0 => {
                println!("\nGoodbye!\n");
                exit(0);
            },
            1 => {
                create_bot(&mut manager);
            },
            2 => {
                view_bots(&mut manager);
            },
            _ => ()
        }
        
    }
}

fn get_user_input(prompt: &str) -> String {
    // Prompting user
    print!("{} > ", prompt);
    stdout().flush().expect("Failed to flush stdout");

    // To obtain user input, we need to allocate space for the input first
    let mut buffer = String::new();

    // Now we are locking `stdin` and rerouting its input to a variable
    stdin().read_line(&mut buffer).expect("Failed to read user input");

    // Trimming white space and returning the string
    buffer.trim().to_string()
}

fn prompt_choice(prompt: &str) -> i32 {
    let val = get_user_input(prompt);

    match val.parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("\nError: entered an invalid number.\n");
            prompt_choice(prompt)
        }
    }
}

fn prompt_bot_type() -> BotType {
    let mut prompt = format!("\nValid Types:\n1) Unipedal\n2) Bipedal\n3) Quadrupedal\n");
    prompt.push_str(&format!("4) Arachnid\n5) Radial\n6) Aeronautical\n\nEnter a bot type"));

    let mut choice = prompt_choice(prompt.as_str());

    while !(choice >= 1 && choice < 7) {
        println!("\n* Enter a valid type!\n");
        choice = prompt_choice("Pick a menu option");
    }

    match choice {
        1 => BotType::Unipedal,
        2 => BotType::Bipedal,
        3 => BotType::Quadrupedal,
        4 => BotType::Arachnid,
        5 => BotType::Radial,
        6 => BotType::Aeronautical,
        _ => BotType::Aeronautical
    }
}

fn display_bots(bots: &Vec<Robot>) {
    let mut i = 1;

    println!("\nBots:");
    for bot in bots.iter() {
        println!("{}) {} ({})", i, bot.get_name(), bot.get_type());
        i += 1;
    }
    println!();
}

fn create_bot(manager: &mut BotManager) {
    let name = get_user_input("\nEnter a bot name");
    let bot_type = prompt_bot_type();

    manager.add_bot(name.as_str(), bot_type);
}

fn view_bots(manager: &mut BotManager) {
    let bots = manager.get_bots();
    display_bots(bots);

    let mut bot = prompt_choice("Pick a Bot or Enter '0' to Cancel");

    while !(bot >= 0 && bot <= (bots.len() as i32)) {
        println!("\n* Pick a valid option!\n");
        bot = prompt_choice("Pick a Bot or Enter '0' to Cancel");
    }

    if bot > 0 {
        bot -= 1;
        bots[bot as usize].display_tasks();
        let mut task = prompt_choice("Pick a Task or Enter '0' to Cancel");

        while !(task >= 0 && task <= bots[bot as usize].get_num_tasks()) {
            println!("\n* Pick a valid option!\n");
            task = prompt_choice("Pick a Task or Enter '0' to Cancel");
        }

        if task > 0 {
            task -= 1;
            manager.call_to_work(bot, task);
        }
    }
}
