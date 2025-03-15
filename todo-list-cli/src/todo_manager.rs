use std::fs;


use crate::model::{Owner, Task, TodoFile};


#[allow(dead_code)]
pub fn uuid_generator() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

#[allow(dead_code)]
pub fn read_todo_file() -> TodoFile {
    let file_path = "todo.json";

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    
    let todo_file: TodoFile = serde_json::from_str(&contents).unwrap();

    todo_file
}

#[allow(dead_code)]
pub fn updated_todo_file(todo_file: &TodoFile) {
    let file_path = "todo.json";

    let json = serde_json::to_string_pretty(todo_file).unwrap();

    fs::write(file_path, json).expect("Unable to write file");
}

pub fn clean_todo_file() {
    let file_path = "todo.json";

    let todo_file = TodoFile {
        owner: vec![],
    };

    let json = serde_json::to_string_pretty(&todo_file).unwrap();

    fs::write(file_path, json).expect("Unable to write file");
}


#[allow(dead_code)]
pub fn get_owner_by_name<'a>(todo_file: &'a TodoFile, name: &'a str) -> Option<&'a Owner> {
    todo_file.owner.iter().find(|owner| owner.name == name)
}

#[allow(dead_code)]
pub fn get_owner_by_uuid<'a>(todo_file: &'a TodoFile, uuid: &'a str) -> Option<&'a Owner> {
    todo_file.owner.iter().find(|owner| owner.uuid == uuid)
}

#[allow(dead_code)]
pub fn delete_owner_by_name(todo_file: &mut TodoFile, name: &str) {
    todo_file.owner.retain(|owner| owner.name != name);
}

#[allow(dead_code)]
pub fn delete_owner_by_uuid(todo_file: &mut TodoFile, uuid: &str) {
    todo_file.owner.retain(|owner| owner.uuid != uuid);
}

#[allow(dead_code)]
pub fn add_owner(todo_file: &mut TodoFile, owner: Owner, tasks: Vec<Task>) {
    todo_file.owner.push(Owner {
        uuid: owner.uuid,
        name: owner.name,
        email: owner.email,
        tasks: tasks,
    });
}

#[allow(dead_code)]
pub fn add_task(todo_file: &mut TodoFile, owner_uuid: &str, task: Task) {

    let owner = get_owner_by_uuid(todo_file, owner_uuid).unwrap();
    let owner_index = todo_file.owner.iter().position(|o| o.uuid == owner.uuid).unwrap();
    let task_uuid = uuid_generator();

    todo_file.owner[owner_index].tasks.push(Task {
        uuid: task_uuid,
        title: task.title,
        completed: task.completed,
    });
}