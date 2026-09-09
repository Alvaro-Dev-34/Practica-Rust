fn main() {
    //practica: struct
    let legolas = Personaje{vida:50, danio:10, mana:20.7};

    println!("Legolas:");
    println!("Vida: {:?}", legolas.vida);
    println!("Daño: {}", legolas.danio);
    println!("Mana: {}", legolas.mana);

}

struct Personaje {
    vida: i32,
    danio: i32,
    mana: f64,
}