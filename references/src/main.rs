fn str_len(s: &String) -> usize {
    // not change the string "s"
    let aux: usize = s.len();
    aux
}

fn change_string(s: &mut String) {
    // change the string "s"
    s.push_str(" World!")
}

fn main() {
    // referencia
    let s = String::from("Hello World!");
    let length = str_len(&s); // se le pasa el argumento como referencia
    println!("the length of the string ({}), is {}", s, length);

    // referencia mutables
    let mut h = String::from("Hello");
    change_string(&mut h);
    println!("{}", h);
}
