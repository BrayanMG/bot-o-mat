mod robot;
mod task;
mod bot_type;
mod bot_manager;

use crate::bot_type::{BotType, from_str};
// use crate::robot::Robot;
use crate::bot_manager::BotManager;

// use::std::thread;
use std::io::{self, Write, stdout, stdin};
use::std::process::exit;

fn main() {
    let mut manager = BotManager::new();
    let menu = "\nMain Menu\n---------\n0) Quit\n1) Create a Bot\n2) View Bots\n";
    let mut choice = -1;

    loop {
        println!("{}", menu);
        choice = prompt_choice("Pick a menu option");

        match choice {
            0 => {
                println!("\nGoodbye!\n");
                exit(0);
            },
            1 => {
                let name = get_user_input("Enter a bot name");
                let bot_type = from_str(get_user_input("Enter a bot type").as_str());

                manager.add_bot(name.as_str(), bot_type);
            },
            2 => {
                manager.display_bots();
                let bot = prompt_choice("Pick a Bot");

                manager.show_tasks(bot);
                let task = prompt_choice("Pick a Task");

                manager.call_to_work(bot, task);
            }
            _ => {
                while !(choice >= 0 && choice < 3) {
                    println!("\nPick a valid menu option!\n");
                    choice = prompt_choice("Pick a menu option");
                }
            }
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
