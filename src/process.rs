use term::{
    self,
    color,
};
use std::ops::{
    Deref,
};
use std::process;

use errors::{
    Result,
    Error,
};

#[derive(Clone)]
pub struct Process {
    exec: fn() -> Result<()>,
}

impl Process {
    pub fn new(exec: fn() -> Result<()>) -> Process {
        Process {
            exec: exec,
        }
    }

    pub fn execute(&self) -> ProcessResult {
        ProcessResult((self.exec)())
    }
}

pub struct ProcessResult(Result<()>);

impl ProcessResult {
    pub fn handle(self) {
        if let Err(err) = self.0 {
            if let Some(mut term) = term::stdout() {
                let _ = term.fg(color::RED);
                let _ = writeln!(term, "Encountered an error:\n");
                let _ = writeln!(term, "{}", err);
            } else {
                println!("Encountered an error:\n");
                println!("{}", err);
            }

            process::exit(1);
        }
    }
}

impl Deref for ProcessResult {
    type Target = Result<()>;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}
