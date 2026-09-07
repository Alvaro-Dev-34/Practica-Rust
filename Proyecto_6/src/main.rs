
//Practica: enum + struct

enum Sabor {
    Amargo,
    Dulce,
    Salado,
}

struct Bebida {
    sabor: Sabor,
    porcentaje_azucar: f64,
}

fn print_bebida(bebida:Bebida){
    match bebida.sabor {
        Sabor::Amargo => println!("Sabor: Amargo"),
        Sabor::Dulce => println!("Sabor: Dulce"),
        Sabor::Salado => println!("Sabor: Salado"),
    }

    println!("porcentaje: {}%", bebida.porcentaje_azucar);
}

fn main() {
    let bebida_dulce = Bebida{sabor:Sabor::Dulce, porcentaje_azucar:67.0};
    print_bebida(bebida_dulce);

    let bebida_amargo = Bebida{sabor:Sabor::Amargo, porcentaje_azucar:5.1234567890123456789};
    print_bebida(bebida_amargo);
}

