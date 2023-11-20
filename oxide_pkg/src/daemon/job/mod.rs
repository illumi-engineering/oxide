pub mod manager;

pub trait Job {
    fn run();
}

pub trait Task {
    fn run(self, id: i64);
}

pub enum Action {
    RunShellCommand(String)
}

impl Task for Action {
    fn run(self, id: i64) {
        match self {
            Action::RunShellCommand(command) => {

            },
            _ => {

            }
        }
    }
}