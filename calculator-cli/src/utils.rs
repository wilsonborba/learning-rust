use std::{any::type_name, io};

#[allow(dead_code)]
pub fn is_number<T>(_value: &T) -> bool {
    let type_name = type_name::<T>();
    type_name == "i32" || type_name == "f64" || type_name == "u32" || type_name == "i64" // Extend as needed
}


#[allow(dead_code)]
pub fn input(msg: &String) -> f64 {
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
                println!("Please enter a valid value:");
            }
        }
    }

    input_string.trim().parse::<f64>().unwrap()


}

#[allow(dead_code)]
pub fn get_menu_choice() -> f64 {

    let available_choices = vec![1, 2, 3, 4, 5, 6, 7];

    for choice in available_choices {
        println!("{}: {}", choice, match choice {
            1 => "Add",
            2 => "Subtract",
            3 => "Multiply",
            4 => "Divide",
            5 => "Modulo",
            6 => "Power",
            7 => "Exit",
            _ => "Invalid choice"
        });
    }

    let msg = "Please enter your choice: ".to_string();

    let mut choose = input(&msg);



    while choose < 1.0 || choose > 7.0 {
        println!("Invalid choice. Please enter a valid choice.");
        choose = input(&msg);
    }
    
    choose
}