use std::{collections::HashMap, hash::Hash};
fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Red"), 20);

  let score = scores.get("Blue").copied().unwrap_or(0);

  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  let mut field_name = String::from("fav col");
  let field_value = String::from("blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  //Cant do that, hash map take owenrship
// println!({field_name})


// overriding 

  let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
        println!("{scores:?}");

        // adding value when not exists
        scores.entry(String::from("Blue")).or_insert(50);
        scores.entry(String::from("Red")).or_insert(50);
           println!("{scores:?}");
        // update a values based on old value

    let mut map = HashMap::new();
    let text = "Hello world one world2";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
        println!("{map:?}");
  
}
