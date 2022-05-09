
#[derive(Debug)]

pub struct Task {
    description: String,
    eta: u32
}

impl Task {
    pub fn new(description: String, eta: u32) -> Task {
        Task {
            description,
            eta
        }
    }

    pub fn make_copy(&self) -> Task {
        Task{
            description: self.description.clone(),
            eta: self.eta
        }
    }
}
