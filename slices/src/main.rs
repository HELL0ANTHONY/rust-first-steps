fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // si encuentra un espacio vacío es porque comienza otra palabra
        if item == b' ' {
            return &s[..i]; // hace un slice desde la pos: 0 hasta la pos de "i"
        }
    }
    // devuelve todo el string en caso de que sea una sola palabra
    &s[..] // from 0 to (s.len())
}

fn main() {
    let greet = String::from("Hello friends");

    let hello = &greet[0..5]; // Hello
    let hello_2 = &greet[..5]; // Hello
    println!("{}, {}", hello, hello_2);

    let friends = &greet[6..greet.len()]; // friends
    let friends_2 = &greet[6..]; // friends
    println!("{}, {}", friends, friends_2);

    println!("the first word is: '{}'", first_word(&greet));

    // with an array
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3];
    assert_eq!(arr_slice, [2, 3]); // compara el valor de la der con el de la izq
    println!("{:?}", arr_slice);
}
