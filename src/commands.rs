use crate::storage::TaskStorage;
use colored::*;

pub fn add_task(description: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = TaskStorage::load()?;
    let id = storage.add_task(description.to_string());
    storage.save()?;

    println!(
        "{} Task added with ID: {}",
        "✓".green().bold(),
        id.to_string().cyan().bold()
    );
    Ok(())
}

pub fn list_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let storage = TaskStorage::load()?;
    let tasks = storage.get_tasks();

    if tasks.is_empty() {
        println!(
            "{}",
            "No tasks found. Add some tasks with 'add' command.".yellow()
        );
        return Ok(());
    }

    println!("{}", "📋 Task List:".blue().bold());
    println!();

    for task in tasks {
        let checkbox = if task.is_done() {
            format!("[{}]", "x".green().bold())
        } else {
            format!("[{}]", " ".normal())
        };

        let id_str = task.id.to_string();
        let colored_id = if task.is_done() {
            id_str.green().strikethrough()
        } else {
            id_str.cyan().bold()
        };

        let description = if task.is_done() {
            task.description.green().strikethrough()
        } else {
            task.description.white().bold()
        };

        println!("{} {} - {}", checkbox, colored_id, description);

        // Show creation and completion dates
        let created = task
            .created_at
            .format("%Y-%m-%d %H:%M")
            .to_string()
            .bright_black();
        print!("    Created: {}", created);

        if let Some(completed) = task.completed_at {
            let completed_str = completed
                .format("%Y-%m-%d %H:%M")
                .to_string()
                .bright_black();
            println!(" | Completed: {}", completed_str);
        } else {
            println!();
        }
        println!();
    }

    // Show summary
    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.is_done()).count();
    let pending = total - completed;

    println!("{}", "📊 Summary:".blue().bold());
    println!(
        "Total: {} | Completed: {} | Pending: {}",
        total.to_string().cyan(),
        completed.to_string().green(),
        pending.to_string().red()
    );

    Ok(())
}

pub fn complete_task(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = TaskStorage::load()?;

    match storage.find_task_mut(id) {
        Some(task) => {
            if task.is_done() {
                println!(
                    "{} Task {} is already completed!",
                    "⚠".yellow(),
                    id.to_string().cyan()
                );
            } else {
                task.complete();
                storage.save()?;
                println!(
                    "{} Task {} marked as completed!",
                    "✓".green().bold(),
                    id.to_string().cyan().bold()
                );
            }
        }
        None => {
            println!(
                "{} Task with ID {} not found!",
                "✗".red().bold(),
                id.to_string().cyan()
            );
            return Err("Task not found".into());
        }
    }

    Ok(())
}

pub fn delete_task(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = TaskStorage::load()?;

    if storage.remove_task(id) {
        storage.save()?;
        println!(
            "{} Task {} deleted!",
            "🗑".normal(),
            id.to_string().cyan().bold()
        );
    } else {
        println!(
            "{} Task with ID {} not found!",
            "✗".red().bold(),
            id.to_string().cyan()
        );
        return Err("Task not found".into());
    }

    Ok(())
}

pub fn clear_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = TaskStorage::load()?;
    let count = storage.get_tasks().len();

    if count == 0 {
        println!("{}", "No tasks to clear.".yellow());
        return Ok(());
    }

    storage.clear_tasks();
    storage.save()?;

    println!(
        "{} Cleared {} task(s)!",
        "🧹".normal(),
        count.to_string().cyan().bold()
    );
    Ok(())
}
