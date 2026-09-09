/*
parametros funciones:

MOVE: las variables cambian de propietario de una funcion a otra
y deja de pertenecer a la anterior funcion (por lo que no se puede usar en esa funcion)

BORROW: se usa &variable, para indicar que la variable se "presta" en vez de "mover"

*/

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull =>  println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}
