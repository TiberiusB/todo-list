mod cli; // "mod" veut dire module, est un keyword qui permet d'importer/exporter des modules, d'autres fishiers ou ensemble de fichiers (modules ou sous modules) qu'on cre dans le src, qui est le dossier source.
mod error;
mod file;
mod tasks;
mod todo_list;

use clap::Parser;
use cli::*;
use error::TodoResult;
use file::loadjson; // "*" means export all. If not, we can write "use cli::Person" or "use cli::{PErson,Age}"

fn main() -> TodoResult<()> {
    let cli = Cli::parse();
    let todo_list = loadjson()?;
    dbg!(todo_list);

    match cli.command {
        Command::View => todo!("View all tasks"),
        Command::Add(args) => println!("{:?}", args),
        Command::Remove(args) => println!("{:?}", args),
        Command::Update(args) => println!("{:?}", args),
        Command::Complete(args) => println!("{:?}", args),
        Command::Uncomplete(args) => println!("{:?}", args),
        Command::Status(args) => println!("{:?}", args),
        Command::ClearCompleted => println!("ccccc"),
        Command::CealrAll => println!("cccccc"),
    }
    Ok(())
}
