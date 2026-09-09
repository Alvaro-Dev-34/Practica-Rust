use std::io;

fn main() {
    let rol_texto = introducirRol();
    let rol_enum = match rol_texto.as_str() {
        "admin" => Roles::Admin,
        "dev" => Roles::Dev,
        "qa" => Roles::Qa,
        _ => {eprintln!("ERROR: rol no reconocido");
                eprintln!("Cerrando programa...");
                std::process::exit(1);},
    };

    let acceso_rol = match rol_enum {
        Roles::Admin => true,
        Roles::Dev => false,
        Roles::Qa => false,
    };

    println!("El rol {} tiene acceso: {}", rol_texto, acceso_rol)
}

enum Roles {
    Admin,
    Dev,
    Qa,
}

//introduzco rol por consola
fn introducirRol() -> String {
    println!("Por favor, introduzca el rol:");
    let mut input = String::new();
    
    // Lee la entrada del usuario en la variable 'input'
    io::stdin().read_line(&mut input)
        .expect("Fallo al leer linea");

    input.trim().to_lowercase().to_string()
}