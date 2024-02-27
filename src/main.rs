use clap::{command, Arg, Command};
use std::collections::HashMap;

fn main() {
    let mut choices = HashMap::new();
    choices.insert("o", "Optional");
    choices.insert("l", "Low");
    choices.insert("m", "Medium");
    choices.insert("h", "High");
    choices.insert("c", "Crucial");

    let match_result = command!()
        .about("This application acts as a to-do list")
        .subcommand(
            Command::new("add-todo")
                        .about("Adds a new todo item")
                        .arg(
                            Arg::new("priority")
                                .value_parser(["o", "l", "m", "h", "c"])
                                .default_value("m")
                                .help("The priority level of the todo item")
                                
                        )
                        .arg(
                            Arg::new("todo-name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .help("The name of the todo item")
                        )
                        .arg(
                            Arg::new("file-name")
                                .short('f')
                                .long("file-name")
                                .default_value("mytodos.txt")
                                .help("The file in which the todos will be saved")
                        )
                        .arg(
                            Arg::new("description")
                                .short('d')
                                .long("description")
                                .required(true)
                                .help("The description of the todo item")
                        )
        )
        .subcommand(
            Command::new("edit-todo")
                        .about("Edits a todo item")
                        .arg(
                            Arg::new("priority")
                                .value_parser(["o", "l", "m", "h", "c"])
                                .default_value("m")
                                .required(true)
                                .help("The priority level of the todo item")
                        )
                        .arg(
                            Arg::new("todo-name")
                                // .value_delimiter(',')
                                .short('n')
                                .long("name")
                                .required(true)
                                .help("The name of the todo item")
                        )
                        .arg(
                            Arg::new("file-name")
                                .short('f')
                                .long("fname")
                                .required(true)
                                .help("The name of the file")
                        )
                        .arg(
                            Arg::new("index")
                                .short('i')
                                .long("idx")
                                .required(true)
                                .help("The index of the todo item")
                        )
                        .arg(
                            Arg::new("description")
                                .short('d')
                                .long("desc")
                                .required(true)
                                .help("The description of the todo item")
                        )
        )
        .subcommand(
            Command::new("delete-todo")
                        .about("Deletes a todo item")
                        .arg(
                            Arg::new("index")
                                .short('i')
                                .long("idx")
                                .required(true)
                                .help("The index of the todo item")
                        )
                        .arg(
                            Arg::new("file-name")
                                .short('f')
                                .long("fname")
                                .required(true)
                                .help("The name of the file")
                        )
        )
        .subcommand(
                Command::new("list-todos")
                        .about("Lists all todo items")
                        .arg(
                            Arg::new("priority")
                                .value_parser(["o", "l", "m", "h", "c"])
                                .help("The priority level of the todo item")
                        )  
                        .arg(
                            Arg::new("file-name")
                                .short('f')
                                .long("fname")
                                .required(true)
                                .help("The name of the file")
                        )  
        )
        .get_matches();

    if let Some(val) = match_result.subcommand_name(){
        if val == "add-todo" {
            to_do::todo_operations::add_todo(match_result, choices)
        } else if val == "edit-todo" {
            to_do::todo_operations::edit_todo(match_result, choices)
        } else if val == "delete-todo" {
            to_do::todo_operations::delete_todo(match_result)
        } else if val == "list-todos" {
            to_do::todo_operations::list_todos(match_result, choices)
        }
    }
}