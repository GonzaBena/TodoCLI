use std::collections::HashMap;
use std::io::{self, Write};

use colored::Colorize;

pub struct User {
    pub name: String,
    pub pass: String,
}

pub struct UserList {
    pub users: HashMap<String, String>
}

pub struct Task {
    pub id: u32,
    pub description: String,
    pub user: User,
    pub status: bool,
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl User {
    pub fn new(name: String, pass: String) -> User {
        User { name, pass }
    }

    pub fn change_pass(user: User, pass: String) -> User {
        User { pass, ..user }
    }
}

impl UserList {
    pub fn new() -> UserList {
        UserList { users: HashMap::new() }
    }

    pub fn login(&self) -> bool {
        let mut username = String::new();
        let mut password = String::new();

        println!("{}", "\
+-------------------------+
|                         |
|          Login          |      
|                         |
+-------------------------+
".blue());

        print!("Ingrese su usuario: ");
        io::stdout().flush().unwrap(); // Libera la retencion del buffer y permite imprimir lo de arriba
        io::stdin()
            .read_line(&mut username)
            .expect("Error al Recibir el usuario");
        
        print!("Ingrese su contrasena: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut password)
            .expect("Error al Recibir el contrasena");

        self.users.iter().find(|x| x.0.to_string() == username && x.1.to_string() == password).is_some()
    }

    pub fn register(&mut self) -> bool {
        let mut username = String::new();
        let mut password = String::new();

        println!("{}", "\
+----------------------------+
|                            |
|          Register          |      
|                            |
+----------------------------+
".blue());

        print!("Ingrese su usuario: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut username)
            .expect("Error al Recibir el usuario");
        
        print!("Ingrese su contrasena: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut password)
            .expect("Error al Recibir el contrasena");

        self.users.insert(username, password);
        true
    }
}

impl Task {
    pub fn new(id: &mut u32, description: String, user: User, status: bool) -> Task {
        *id += 1;
        Task { id: *id - 1, description, user, status }
    }

    pub fn belongs_to(&self, user: &User) -> bool {
        self.user.name == user.name && self.user.pass == user.pass
    }
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn from(tasks: Vec<Task>) -> TodoList {
        TodoList { tasks }
    }

    pub fn insert(&mut self, task: Task) {
        self.tasks.push(task)
    }

    pub fn show_all(&self) {
        for task in self.tasks.iter() {
            println!("{} \n id: {} ", '{', task.id);
            println!(" description: {} ", task.description);
            println!(" user: {}", task.user.name);
            println!(" status: {}", task.status.to_string());
            println!("{},", '}')
        }
    }

    pub fn find_all(&self, user: &User) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|x| x.belongs_to(&user))
            .collect()
    }

    pub fn find_one(&self, user: &User) -> &Task {
        self.tasks.iter().find(|x| x.belongs_to(user)).unwrap()
    }
}

pub fn welcome() {
    println!("\
+-----------------------------------------------+
|                                               |
|          Welcome to de ToDo List App          |      
|                                               |
+-----------------------------------------------+
"); 
}


