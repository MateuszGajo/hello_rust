fn main() {
    println!("Hello, world!");

    let mut s = String::from("fds");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {s2}");
    s.push('L');
    let s2 = String::from("dsdd");
    // fn add(self, s: &str) -> String { rust uses deref coerion, which turns &s into &s2[]
    let s3 = s + &s2[..];


    println!("value of s2 {s2}");

    // In rust u cant access string by index
    // s3[1];
    // not all strings are equal, e.g russian characters are store in two bytes, so what at index [0] u dont exatcly have "З"
//     let hello = "Здравствуйте";
// let answer = &hello[0];

let hello = "Здравствуйте";

let s = &hello[0..4];
println!("string s {s}");

for b in "Здравствуйте".bytes() {
    println!("{b}")
}

for c in "Здравствуйте".chars() {
    println!("{c}")
}
}
