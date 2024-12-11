use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "TiberiusB", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// View all tasks
    View,
    /// Add a task
    Add(AddTaskArgs),
    /// Remove a task
    Remove(TaskNameArg),
    /// Update a task
    Update(UpdateTaskArgs),
    /// Mark a task Completed
    Complete(TaskNameArg),
    /// Mark a Task as Uncompleted
    Uncomplete(TaskNameArg),
    /// Show the status of a Task
    Status(TaskNameArg),
    /// Clear completed Tasks
    ClearCompleted,
    /// Clear all tasks
    CealrAll,
}

#[derive(Debug, Args)]
pub struct AddTaskArgs {
    pub name: String,
    pub description: Option<String>,
    pub date: Option<String>,
}
#[derive(Debug, Args)]
pub struct TaskNameArg {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct UpdateTaskArgs {
    /// Task name
    pub name: String,
    /// New task name
    #[clap(long, short)]
    pub new_name: Option<String>,
    /// Task description
    #[clap(long, short)]
    pub description: Option<String>,
    /// TAsk date
    #[clap(long)]
    pub date: Option<String>,
    /// Task completed
    #[clap(long, short)]
    pub completed: Option<bool>,
}
