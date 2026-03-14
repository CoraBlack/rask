use std::{fs, path::PathBuf};

use anyhow::{Ok, bail};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskJson {
    name: String,
    command: String,
    args: Option<Vec<String>>,
    // cwd: Option<PathBuf>,
}

impl TaskJson {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let json_str = fs::read_to_string(path)?;
        let task: TaskJson = serde_json::from_str(&json_str)?;
        Ok(task)
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut cmd = std::process::Command::new(&self.command);
        if self.args.is_some() {
            cmd.args(self.args.as_ref().unwrap());
        }

        let exec_status = cmd.status()?;
        if !exec_status.success() {
            bail!("Command exit abnormally!");
        }

        Ok(())
    }
}
