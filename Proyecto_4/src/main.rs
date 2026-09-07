fn main() {
    println!("INIT");
    // practica tipo de dato: enum

    let color = Color::Amarillo;
    
    mostrar_color(color);


}

fn mostrar_color(col:Color) {
     match col {
        Color::Rojo => println!("Print del color ROJO"),
        Color::Verde => println!("Print del color VERDE"),
        Color::Morado => println!("Print del color MORADO"),
        Color::Azul => println!("Print del color AZUL"),
        Color::Amarillo => println!("Print del color AMARILLO"),
        Color::Marron => println!("Print del color MARRON"),
    }
}

enum Color {
    Rojo,
    Verde,
    Morado,
    //el resto no se usan
    Azul,
    Amarillo,
    Marron,
}