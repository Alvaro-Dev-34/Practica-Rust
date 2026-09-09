fn main() {
    //practica: Tuplas

    let x = 1;
    let y = 2;
    let z = "texto";

    metodo_tuplas(x,y,z);

}

fn metodo_tuplas(x: i32, y: i32, z: &str) {
    let t = (x,y,z);
    println!("X: {}", t.0);
    println!("Y: {}", t.1);
    println!("Z: {}", t.2);

    //paso valores de una tupla a variables i32
    let (a,b,c) = t;
    println!("A: {}", a);
    println!("B: {}", b);
    println!("C: {}", c);
}
