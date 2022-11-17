use std::io;
use std::io::Write;
use std::process;

use todo_cli::*;
use colored::*;

fn exception(mensaje: &str) -> bool {
    eprintln!("\n{}", mensaje.red() );
    process::exit(1);
}

fn main() {
    // Variables
    let mut id: u32 = 0;
    let mut todos: TodoList = TodoList::new();
    let mut users: UserList = UserList::new();
    let mut opcion: String = String::new();
    let int_opcion: u8;

    let logged: bool;

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
        _ => exception("La opcion debe ser 1 o 2"),
    };

    if !logged {
        exception("El usuario que ingreso no existe");
    }

    todos.insert(
        Task::new(
            &mut id, // id 
            String::from("Hola Mundo"), // description
            User::new("Gonza".to_string(), "bena".to_string()), // User 
            false) // status
        ); 

}
