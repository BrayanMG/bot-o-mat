
#[derive(Debug, Clone)]

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

    pub fn get_desc(&self) -> &str {
        return self.description.as_str();
    }

    pub fn get_eta(&self) -> u32 {
        return self.eta;
    }
}
