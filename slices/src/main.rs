fn main() {
    println!("Hello, world!");
    let mut s = String::from("aa bb");
    let looking = first_word(&s);

    println!("the first word is {looking}");
    
    let looking = first_word_2(&s);
    println!("the first word is {looking}");

    // let index = first_word_index(&s);
    // now there would be a problem, index is 5, but the word is empty
    // s.clear();
    // solution???? string slices

    let slice = &s[0..2];
    println!("slice? {slice}");
    let slice = &s[..];
    println!("slice? {slice}");

    // s.clear();

}


fn first_word(s: &String) -> String {
    let mut words = s.split(" ");

    words.next().unwrap().to_string()
}

// fn first_word_index(s: &String) -> usize {
//     let  bytes = s.as_bytes();


//     for (i, &item) in bytes.iter().enumerate() {
//             println!("hello {item}");
//         if item == b' '{
//             return i;
//     }
// }

//     s.len()
// }

fn first_word_2(s: &String) -> &str {
        let  bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i]
    }
}
    &s[..]
}