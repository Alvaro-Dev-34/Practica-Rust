fn main() {
    let anios = 2;
    let dias: i32 = yearsToDays(anios);
    println!("Dias: {}",dias);
}

fn yearsToDays(anios: i32) -> i32{
    anios * 365
}