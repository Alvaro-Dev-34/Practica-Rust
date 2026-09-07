fn main() {

    //PRACTICA: println
    let mut num  = 0;
    loop {
        metodo(7, "alvaro", 'c');
        num = num + 1;
        if num == 50 {
            break;
        }
    }
    println!("FIN!!!!!")
}

fn metodo (a:i32, b:&str, c:char) -> &str {
    println!("A:{a} | B:{b} | C:{c}");


    "OK"
}
