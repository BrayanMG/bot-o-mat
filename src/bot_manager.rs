use crate::bot_type::BotType;
use crate::task::Task;
use crate::robot::Robot;

use rand::seq::SliceRandom;
use::std::thread;
use::std::time::Duration;

pub struct BotManager {
    bots: Vec<Robot>,
    tasks: Vec<Task>
}

impl BotManager {
    pub fn new() -> BotManager {
        BotManager {
            bots: Vec::new(),
            tasks: create_tasks()
        }
    }

    pub fn add_bot(&mut self, name: &str, bot_type: BotType) {
        let mut bot = Robot::new(String::from(name), bot_type);

        let mut random = rand::thread_rng();
        self.tasks.shuffle(&mut random);

        for i in 0..5 {
            bot.assign_task(self.tasks.get(i).unwrap().clone());
        }
    
        self.bots.push(bot);
    }

    pub fn get_bots(&self) -> &Vec<Robot> {
        &self.bots
    }

    pub fn call_to_work(&mut self, bot: i32, task: i32){
        self.bots[bot as usize].complete_task(task);
        thread::sleep(Duration::from_millis(100)); // for output order
    }
}

fn create_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    tasks.push(Task::new(String::from("do the dishes"), 1000));
    tasks.push(Task::new(String::from("sweep the house"), 3000));
    tasks.push(Task::new(String::from("do the laundry"), 10000));
    tasks.push(Task::new(String::from("take out the recycling"), 4000));
    tasks.push(Task::new(String::from("make a sammich"), 7000));
    tasks.push(Task::new(String::from("mow the lawn"), 20000));
    tasks.push(Task::new(String::from("rake the leaves"), 18000));
    tasks.push(Task::new(String::from("give the dog a bath"), 14500));
    tasks.push(Task::new(String::from("bake some cookies"), 8000));
    tasks.push(Task::new(String::from("wash the car"), 20000));

    tasks
}

