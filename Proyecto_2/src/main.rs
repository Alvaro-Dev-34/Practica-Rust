fn main() {
    let frase = "I found Nemo at [the order of the word you find nemo]!";
    
    let (posicion, palabra) = encontrar_palabra(frase);

    println!("palabra y posicion: {} -> {}", palabra, posicion);

}

fn encontrar_palabra(frase: &str) -> (usize, &str) {
    frase.split_whitespace()
        .enumerate()
        .find(|(_i, w)| w.eq_ignore_ascii_case("nemo"))
        .unwrap_or((0, "no encontrada"))
}
