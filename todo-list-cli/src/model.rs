use serde::Deserialize;



#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Owner {
    pub name: String,
    pub email: String,
}


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TodoFile {
    pub owner: Owner,
    pub tasks: Vec<Task>,
}


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}