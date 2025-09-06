use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const TASKS_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStorage {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}

impl Default for TaskStorage {
    fn default() -> Self {
        TaskStorage {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
}

impl TaskStorage {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if !Path::new(TASKS_FILE).exists() {
            return Ok(TaskStorage::default());
        }

        let file = File::open(TASKS_FILE)?;
        let reader = BufReader::new(file);
        let storage = serde_json::from_reader(reader)?;
        Ok(storage)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(TASKS_FILE)?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn add_task(&mut self, description: String) -> u32 {
        let task = Task::new(self.next_id, description);
        let id = task.id;
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    pub fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    pub fn remove_task(&mut self, id: u32) -> bool {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
