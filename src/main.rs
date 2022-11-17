use std::io;
use std::io::Write;
use std::process;

use todo_cli::*;
use colored::*;

fn main() {
    // Variables
    let mut id: u32 = 0;
    let mut todos: TodoList = TodoList::new();
    let mut users: UserList = UserList::new();
    let mut opcion: String = String::new();
    let int_opcion: u8;

    let mut logged: bool = false;

    // Logica
    welcome();

    println!("Selecciones una opcion: ");
    println!("1 - Iniciar Sesion");
    println!("2 - Registrarse");

    print!("{}", "-> ".green());
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut opcion)
        .expect("Error al recibir la opcion");

    int_opcion = opcion.trim().parse().unwrap();
    
    logged = match &int_opcion {
        1 => users.login(),
        2 => users.register(),
        _ => {
            eprintln!("\n{}", "La opcion debe ser 1 o 2".red() );
            process::exit(1);
        },
    };

    match logged {
        false => eprintln!("\n{}", "El usuario que ingreso no existe".red()),
        true => (),
    }

    todos.insert(
        Task::new(
            &mut id, // id 
            String::from("Hola Mundo"), // description
            User::new("Gonza".to_string(), "bena".to_string()), // User 
            false) // status
        ); 

}
