fn main() {
    //Practica: Expressions
    let num = 6;

    //creo variable booleano en un if, dentro de la declaracion
    let is_5 = if num < 5 {
        true
        }
        else {
            false
        };

    println!("Es menor que 5?: {}", is_5);

    //creo variable booleano en funcion de la opción de ENUM
    let opciones = Opciones::Apagado;

    let is_encendido = match opciones{
        Opciones::Encendido => true,
        Opciones::Apagado => false,
    };

    println!("Esta encendido?: {}", is_encendido);


}

enum Opciones {
    Encendido,
    Apagado,
}