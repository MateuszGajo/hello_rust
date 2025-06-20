fn main() {
    let s1 = String::from("hello");

    let len  = calculate_length(&s1);
   
    println!("the length of '{s1}' is {len}.");


    // let try change String without ownership
    {
        let mut s = String::from("hello");

        // can't do that
        // change_no_owner(s);
        change_no_owner_mutable(&mut s);

        println!("some me variable after its been mutated {s}");
    }
//Mutable references have on big restirction, if you have a mutable reference to a value ,you can have no other refernces o that value
    {
        let mut s = String::from("welcome");

        // can't do that, benefit of having this restriction is prevnting data races, that can occure when:
        // 1. two or more pointer access the same dat at the same time
        // 2. at least one of the pointers is being used to write to the data
        // 3. ther no mechanism being used to synchronise access to the data

        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
    }
     {
        let mut s = String::from("aa");
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;
     }

     {
        // also we can't combine mutable and immutable references

        // let mut s = String::from("aa");
        // let r1 = &s;
        // let r2 = &s;
        // let r3 = &mut s;

        // println!("{}, {}, and {}", r1, r2, r3);
     }

      {
            let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
      }

      {
            let reference_to_nothing = dangle();
      }
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}// s is dropped here and we are trying to return reference

fn calculate_length(s: &String)  -> usize {
     // creates a references but not owns it, points to s1 not to memory location
    s.len()
} // here s goes our of scope. Because s does not have ownership of what is refers to, the value is not dropped.

// cant do that
// fn change_no_owner(s: &String) {
//     s.push_str("ss");
// }

//Mutable references have on big restirction, if you have a mutable reference to a value ,you can have no other refernces o that value
 fn change_no_owner_mutable(s:  &mut String) {
    s.push_str("ss");

}