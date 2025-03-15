use serde::{Deserialize, Serialize};




#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct TodoFile {
    pub owner: Vec<Owner>,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Owner {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Task {
    pub uuid: String,
    pub title: String,
    pub completed: bool,
}