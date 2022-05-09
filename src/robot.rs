use crate::bot_type::BotType;
use crate::task::Task;

#[derive(Debug)]

pub struct Robot {
    name: String,
    bot_type: BotType,
    tasks: Vec<Task>
}

impl Robot {
    pub fn new(name: String, bot_type: BotType) -> Robot {
        Robot {
            name,
            bot_type,
            tasks: Vec::new()
        }
    }

    pub fn assign_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
