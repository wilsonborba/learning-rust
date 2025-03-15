use std::{fs, io};

use crate::{model, todo_manager};



#[allow(dead_code)]
pub fn input_number(msg: &String) -> f64 {
    let mut input_string = String::new();
    let mut is_it_number: bool = false;
    println!("{}", msg);
    while !is_it_number {
        input_string.clear();
        io::stdin().read_line(&mut input_string).expect("Failed to read line...");
        match input_string.trim().parse::<f64>() {
            Ok(_) => {
                
                is_it_number = true;
            },
            Err(_) => {
                println!("Please enter a valid number value:");
            }
        }
    }

    input_string.trim().parse::<f64>().unwrap()


}


#[allow(dead_code)]
pub fn input_bool(msg: &String, prevent_empty: bool) -> bool {
    let mut input_string = String::new();
    let mut is_it_bool: bool = false;
    println!("{}", msg);


    // if prevent_empty is true, then we need to check if the input is boolean
    if prevent_empty {
        while !is_it_bool {
            input_string.clear();
            io::stdin().read_line(&mut input_string).expect("Failed to read line...");
            match input_string.trim().parse::<bool>() {
                Ok(_) => {
                    is_it_bool = true;
                },
                Err(_) => {
                    println!("Please enter a valid boolean value:");
                    input_string.clear();
                    io::stdin().read_line(&mut input_string).expect("Failed to read line...");
                    

                }
            }
        }

        return input_string.trim().parse::<bool>().unwrap();
        
    } else {
        // if prevent_empty is false, then we can accept empty input only
        input_string.clear();
        io::stdin().read_line(&mut input_string).expect("Failed to read line...");

        if input_string.trim().is_empty() {
            return false;
        } else {
            while !is_it_bool {
                match input_string.trim().parse::<bool>() {
                    Ok(_) => {
                        is_it_bool = true;
                    },
                    Err(_) => {
                        println!("Please enter a valid boolean value:");
                        input_string.clear();
                        io::stdin().read_line(&mut input_string).expect("Failed to read line...");
                    }
                }
            }

            return input_string.trim().parse::<bool>().unwrap();

        }
    }
    
    

}


#[allow(dead_code)]
pub fn input_string(msg: &String, prevent_empty: bool) -> String {
    let mut input_string = String::new();

    if prevent_empty {
        while input_string.trim().is_empty() {  
            println!("{}", msg);
            input_string.clear(); 
            io::stdin().read_line(&mut input_string).expect("Failed to read line...");
        }
    } else {
        println!("{}", msg);
        input_string.clear(); 
        io::stdin().read_line(&mut input_string).expect("Failed to read line...");
    }

    input_string.trim().to_string()
}



#[allow(dead_code)]
pub fn add_new_owner() -> (String, String, String) {
    let name = input_string(&"Enter owner name: ".to_string(), true);
    let email = input_string(&"Enter owner email: ".to_string(), true);
    let uuid = uuid::Uuid::new_v4().to_string();
    (name, email, uuid)
}


#[allow(dead_code)]
pub fn update_owner() -> (String, String, String) {
    let uuid = input_string(&"Enter owner UUID: ".to_string(), true);
    let name = input_string(&"Enter new owner name (empty to keep the same): ".to_string(), false);
    let email = input_string(&"Enter new owner email (empty to keep the same): ".to_string(), false);
    (uuid, name, email)
}

#[allow(dead_code)]
pub fn delete_owner() -> String {
    input_string(&"Enter owner UUID: ".to_string(), true)
}

#[allow(dead_code)]
pub fn show_all_owners() {
    let file_path = "todo.json";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let todo_file: model::TodoFile = serde_json::from_str(&contents).unwrap();


    for owner in &todo_file.owner {
        // print first a spacer for readability with "=" character repeated 50 times
        println!("\n");
        println!("{}", "=".repeat(50));
        println!("Owner UUID: {}", owner.uuid);
        println!("Owner Name: {}", owner.name);
        println!("Owner Email: {}", owner.email);
        println!("Owner Tasks: ");
        for task in &owner.tasks {
            println!("Task UUID: {}", task.uuid);
            println!("Task Title: {}", task.title);
            println!("Task Completed: {}", task.completed);
        }
        println!("{}", "=".repeat(50));
        println!("\n");
    }
}

#[allow(dead_code)]
pub fn delete_all_owners( todo_file: &mut model::TodoFile) {
    todo_manager::clean_todo_file();

    todo_file.owner.clear();

    println!("\nAll owners deleted.\n");


}


#[allow(dead_code)]
pub fn add_new_task() -> (String, String, bool) {
    let title = input_string(&"Enter task title: ".to_string(), true);
    let uuid = uuid::Uuid::new_v4().to_string();
    let completed = input_bool(&"Is the task completed? [true/false(default): empty to keep the same]: ".to_string(), false);
    (title, uuid, completed)
}

#[allow(dead_code)]
pub fn update_task() -> (String, String, bool) {

    let uuid = input_string(&"Enter task UUID: ".to_string(), true);
    let title = input_string(&"Enter new task title (empty to keep the same): ".to_string(), false);
    let completed = input_bool(&"Is the task completed? [true/false(default): empty to keep the same]: ".to_string(), false);
    (uuid, title, completed)
}

#[allow(dead_code)]
pub fn delete_task() -> String {
    input_string(&"Enter task UUID: ".to_string(), true)
}

#[allow(dead_code)]
pub fn show_all_tasks_from_owner(owner: &model::Owner) {
    println!("\n");
    println!("Owner UUID: {}", owner.uuid);
    println!("Owner Name: {}", owner.name);
    println!("Owner Email: {}", owner.email);
    println!("Owner Tasks: ");
    for task in &owner.tasks {
        println!("Task UUID: {}", task.uuid);
        println!("Task Title: {}", task.title);
        println!("Task Completed: {}", task.completed);
    }
}

pub fn delete_all_tasks_from_owner(owner: &mut model::Owner) {
    owner.tasks.clear();
}




#[allow(dead_code)]
pub fn show_menu() -> f64 {
    let available_choices = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    println!("\n");
    for choice in available_choices {
        println!("{}: {}", choice, match choice {
            1 => "Add new owner",
            2 => "Update owner",
            3 => "Delete owner",
            4 => "Add new task",
            5 => "Update task",
            6 => "Delete task",
            7 => "Show all owners",
            8 => "Delete all owners",
            9 => "Show all tasks from owner",
            10 =>  "Delete all tasks from owner",
            11 => "Refresh DB",
            12 => "Exit",
            _ => "Invalid choice"
        });
    
    }
    println!("\n");

    let msg = "Please enter your choice: ".to_string();

    let mut choose = input_number(&msg);

    while choose < 1.0 || choose > 12.0 {
        println!("\nInvalid choice. Please enter a valid choice.\n");
        choose = input_number(&msg);
    }

    choose
}


#[allow(dead_code)]
pub fn do_menu_action(todo_file: &mut model::TodoFile, choice: f64) {
    match choice {
        1.0 => {
            let (mut name, mut email, uuid) = add_new_owner();
            
            // check fi already exist a owner with the same name with a loop

            while todo_file.owner.iter().any(|owner| owner.name == name) {
                println!("\nOwner with the same name already exists.\n");
                name = input_string(&"Enter owner name: ".to_string(), true);
            }

            // check if already exist a owner with the same email with a loop
            while todo_file.owner.iter().any(|owner| owner.email == email) {
                println!("\nOwner with the same email already exists.\n");
                email = input_string(&"Enter owner email: ".to_string(), true);
            }

            // then add the owner

            todo_file.owner.push(model::Owner {
                uuid: uuid,
                name: name,
                email: email,
                tasks: vec![],
            });

            todo_manager::updated_todo_file(todo_file);
            println!("\nOwner added.\n");


            

            
        },
        2.0 => {
            let (uuid, name, email) = update_owner();
            let owner = todo_file.owner.iter_mut().find(|owner| owner.uuid == uuid);
            match owner {
                Some(owner) => {
                    if !name.is_empty() {
                        owner.name = name;
                    }
                    if !email.is_empty() {
                        owner.email = email;
                    }

                println!("\nOwner updated.\n");
                },
                None => {
                    println!("\nOwner not found.\n");
                }
            }

            todo_manager::updated_todo_file(todo_file);
            
        },
        3.0 => {
            let uuid = delete_owner();
            todo_file.owner.retain(|owner| owner.uuid != uuid);

            let file_path = "todo.json";
            let json = serde_json::to_string_pretty(todo_file).unwrap();
            fs::write(file_path, json).expect("\nUnable to write file\n");
            println!("\nOwner deleted.\n");
        },
        4.0 => {
            let (title, uuid, completed) = add_new_task();
            let owner_uuid = input_string(&"\nEnter owner UUID: ".to_string(), true);
            let owner = todo_file.owner.iter_mut().find(|owner| owner.uuid == owner_uuid);
            match owner {
                Some(owner) => {
                    owner.tasks.push(model::Task {
                        uuid: uuid,
                        title: title,
                        completed: completed,
                    });

                    todo_manager::updated_todo_file(todo_file);
                    println!("\nTask added.\n");
                },
                None => {
                    println!("\nOwner not found.\n");
                }
            }
        },
        5.0 => {
            let (uuid, title, completed) = update_task();
            let owner_uuid = input_string(&"\nEnter owner UUID: ".to_string(), true);
            let owner = todo_file.owner.iter_mut().find(|owner| owner.uuid == owner_uuid);
            match owner {
                Some(owner) => {
                    let task = owner.tasks.iter_mut().find(|task| task.uuid == uuid);
                    match task {
                        Some(task) => {
                            if !title.is_empty() {
                                task.title = title;
                            }
                            if completed {
                                task.completed = completed;
                            }
                        },
                        None => {
                            println!("\nTask not found.\n");
                        }
                    }
                },
                None => {
                    println!("\nOwner not found.\n");
                }
            }
        },
        6.0 => {
            let uuid = delete_task();
            let owner_uuid =
                input_string(&"\nEnter owner UUID: ".to_string(), true);
            let owner = todo_file.owner.iter_mut().find(|owner| owner.uuid == owner_uuid);  
            match owner {
                Some(owner) => {
                    owner.tasks.retain(|task| task.uuid != uuid);
                },
                None => {
                    println!("Owner not found.");
                }
            }
        },
        7.0 => {
            show_all_owners();
        },
        8.0 => {
            delete_all_owners(todo_file);
        },
        9.0 => {
            let owner_uuid = input_string(&"\nEnter owner UUID: ".to_string(), true);
            let owner = todo_file.owner.iter().find(|owner| owner.uuid == owner_uuid);
            match owner {
                Some(owner) => {
                    show_all_tasks_from_owner(owner);
                },
                None => {
                    println!("\nOwner not found.\n");
                }
            }
        },  
        10.0 => {
            let owner_uuid = input_string(&"\nEnter owner UUID: ".to_string(), true);
            let owner = todo_file.owner.iter_mut().find(|owner| owner.uuid == owner_uuid);
            match owner {
                Some(owner) => {
                    delete_all_tasks_from_owner(owner);
                    todo_manager::updated_todo_file(todo_file);
                    println!("\nAll tasks from owner deleted.\n");
                },
                None => {
                    println!("\nOwner not found.\n");
                }
            }
        },
        11.0 => {
            let refreshed_todo_file = todo_manager::read_todo_file();
            *todo_file = refreshed_todo_file;
            println!("\nDB refreshed.\n");

        }

        12.0 => {
            println!("Exiting...");
            std::process::exit(0);
        },
        _ => {
            println!("Invalid choice.");
        }

    }
}


#[allow(dead_code)]
pub fn run() {

    let file_path = "todo.json";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut todo_file: model::TodoFile = serde_json::from_str(&contents).unwrap();



    loop {
        let choice = show_menu();
        do_menu_action(&mut todo_file, choice);
    }
}






