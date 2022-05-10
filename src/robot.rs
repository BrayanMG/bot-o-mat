use crate::bot_type::BotType;
use crate::task::Task;

use::std::thread;
use::std::time::Duration;
use::std::sync::{Arc, Mutex, Condvar};

#[derive(Debug)]

pub struct Robot {
    name: String,
    bot_type: BotType,
    tasks: Vec<Task>,
    working: Arc<(Mutex<bool>, Condvar)>
}

impl Robot {
    pub fn new(name: String, bot_type: BotType) -> Robot {
        Robot {
            name,
            bot_type,
            tasks: Vec::new(),
            working: Arc::new((Mutex::new(false), Condvar::new()))
        }
    }

    pub fn assign_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, task: i32) {
        let name = self.name.clone();
        let task = self.tasks.remove(task as usize);
        let mutex = self.working.clone();

        thread::spawn(move || {
            let (lock, cvar) = &*mutex;
            let value = lock.lock().unwrap();

            let mut value = cvar.wait_while(value, |working|{
                *working
            }).unwrap();

            *value = true;

            //format!("\n{} Miner notified foreman!\n\n{}", self.m_type.get_value(), "-".repeat(60))
            println!("\n*** {} will now {}! ***\n", name, task.get_desc());
            thread::sleep(Duration::from_millis(task.get_eta() as u64));
            println!("\n*** {} is done with their task! ***\n", name);

            *value = false;
            cvar.notify_one();
            drop(value);
        });
    }

    pub fn display_tasks(&self) {
        let mut i = 0;

        println!("\n{}'s Tasks:", self.name);
        for task in self.tasks.iter() {
            println!("{}) {} (eta: {})", i, task.get_desc(), task.get_eta());
            i += 1;
        }
        println!();
    }

    pub fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn get_type(&self) -> &str {
        return self.bot_type.get_value();
    }
}
