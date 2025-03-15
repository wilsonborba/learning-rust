use std::fs;
mod model;

fn main() {

    let file_path = "todo.json";

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    
    let todo_file: model::TodoFile = serde_json::from_str(&contents).unwrap();

    println!("Todo file: {:?}", todo_file);
    
}
