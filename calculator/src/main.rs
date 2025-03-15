mod operations;
mod utils;


fn main() {
    
    let msg_first_number = "Please enter the first number: ".to_string();
    let msg_second_number = "Please enter the second number: ".to_string();

    let first_number = utils::input(&msg_first_number);

    let second_number = utils::input(&msg_second_number);

    
    let menu_choice = utils::get_menu_choice();

    match menu_choice as i32 {
        1 => println!("Result: {}", operations::add(first_number, second_number)),
        2 => println!("Result: {}", operations::subtract(first_number, second_number)),
        3 => println!("Result: {}", operations::multiply(first_number, second_number)),
        4 => println!("Result: {}", operations::divide(first_number, second_number)),
        5 => println!("Result: {}", operations::modulo(first_number, second_number)),
        6 => println!("Result: {}", operations::power(first_number, second_number)),
        7 => println!("Exiting..."),
        _ => println!("Invalid choice")
    }

    

    



}
