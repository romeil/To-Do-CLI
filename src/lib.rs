pub mod todo_operations {
    use std::collections::HashMap;
    use std::path::Path;
    use std::fs::{self, OpenOptions, File};
    use std::io::{BufRead, BufReader, BufWriter, Write};
    use clap::ArgMatches;


    pub fn add_todo(match_result: ArgMatches, choices: HashMap<&str, &str>) {
        let args = match_result.subcommand_matches("add-todo").unwrap();
    
        let priority = args.get_one::<String>("priority").unwrap().as_str();
        let todo_name = args.get_one::<String>("todo-name").unwrap().replace(",", " ");
        let todo_file = args.get_one::<String>("file-name").unwrap();
        let todo_desc = args.get_one::<String>("description").unwrap().replace(",", " ");
    
        let priority_val = choices[priority];
        let path = Path::new(todo_file);
    
        if !path.exists() {
            File::create(todo_file)
                            .expect("cannot creating file");
    
        }
    
        let mut data_file = OpenOptions::new()
            .append(true)
            .open(todo_file)
            .expect("cannot open file");
        data_file
            .write(format!("{todo_name}: {todo_desc} [PRIORITY: {priority_val}]\n").as_bytes())
            .expect("write failed");
    
    }
    

    pub fn edit_todo(match_result: ArgMatches, choices: HashMap<&str, &str>) {
        let args = match_result.subcommand_matches("edit-todo").unwrap();
    
        let priority = args.get_one::<String>("priority").unwrap().as_str();
        let todo_name = args.get_one::<String>("todo-name").unwrap().replace(",", " ");
        let todo_file = args.get_one::<String>("file-name").unwrap();
        let idx = args.get_one::<String>("index").unwrap();
        let todo_desc = args.get_one::<String>("description").unwrap().replace(",", " ");
    
        let priority_val = choices[priority];
    
        let path = Path::new(todo_file);
        if !path.exists() {
            panic!("this file does not exist");
        }
    
        let mut file_string_content = fs::read_to_string(path).expect("foo");
        file_string_content = file_string_content.replace(",", " ");
    
        let mut string_content_vec: Vec<&str> = file_string_content.split("\n").collect();
    
        let edited_todo = format!("{todo_name}: {todo_desc} [PRIORITY: {priority_val}]");
    
        let idx_to_num: i32 = idx.parse().expect("not a valid number");
        string_content_vec[idx_to_num as usize] = &edited_todo;
        let new_file_string_content = string_content_vec.join("\n");
    
        let file = File::create(todo_file).expect("foo");
        let mut writer = BufWriter::new(file);
    
        writer
            .write(format!("{new_file_string_content}").as_bytes())
            .expect("write failed");
    }
    

    pub fn delete_todo(match_result: ArgMatches) {
        let args = match_result.subcommand_matches("delete-todo").unwrap();
    
        let todo_file = args.get_one::<String>("file-name").unwrap();
        let idx = args.get_one::<String>("index").unwrap();
    
        let path = Path::new(todo_file);
        if !path.exists() {
            panic!("this file does not exist");
        }
    
        let mut file_string_content = fs::read_to_string(path).expect("foo");
        file_string_content = file_string_content.replace(",", " ");
    
        let mut string_content_vec: Vec<&str> = file_string_content.split("\n").collect();
        let idx_to_num: i32 = idx.parse().expect("not a valid number");
        string_content_vec.remove(idx_to_num as usize);
    
        let new_file_string_content = string_content_vec.join("\n");
    
        let file = File::create(todo_file).expect("foo");
        let mut writer = BufWriter::new(file);
    
        writer
            .write(format!("{new_file_string_content}").as_bytes())
            .expect("write failed");
    }
    

    pub fn list_todos(match_result: ArgMatches, choices: HashMap<&str, &str>) {
        let args = match_result.subcommand_matches("list-todos").unwrap();
    
        let todo_file = args.get_one::<String>("file-name").unwrap();
    
        let path = Path::new(todo_file);
        if !path.exists() {
            panic!("this file does not exist");
        }
    
        let file = File::open(todo_file).expect("foo");
        let reader = BufReader::new(file);
    
        match args.get_one::<String>("priority") {
            None => {
                for (idx, line) in reader.lines().enumerate() {
                    if let Ok(line) = line {
                        println!("({}) - {}",idx, line)
                    }
                }
            },
            Some(_)  => {   
                for (idx, line) in reader.lines().enumerate() {
                    if let Ok(line) = line {
                        if line.contains(format!("[PRIORITY: {}", choices[args.get_one::<String>("priority").unwrap().as_str()]).as_str()) {
                            println!("({}) - {}",idx, line);
                        }
                    }
                }
            },
        }
    }
}