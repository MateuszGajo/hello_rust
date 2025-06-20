fn main() {
   let  s = "hello";
   let  mut g = String::from("hello");
   g.push_str(", world!");
   println!("{g}");
   {
    let s = String::from("aa");
    // this scope is over and s in no longer valid, memory its freed
   }

   {
    let x =5;
    let y=x;
   }


   {
    let s1= String::from("hello");
    let s2=s1;

    // s2 takes ownership, we can no longer use s1, its because memory can't be freed two times, so we remove s1 and freed its only from s2 object (both pointing to same memory location)

    println!("${s2}, world");
   }

    {
        let s1  = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }


    {
        let s = String::from("string1");
        let s1 = s.clone();

        takes_ownership(s);

        println!("s should be out of scope {s1}");

        let x =5;

        makes_copy(x);
        println!("x is still accessible {x}")
    }

    {
        let s = gives_ownership();
        let s2 = String::from("hello"); 
        let s2 = transfering_and_returns_ownership(s2);
        println!("taking ownership of s againg {s2}");
    }
}

fn takes_ownership(some_string: String) {
    println!("taking ownership of {some_string}");
} // here some_string goes out of scope, and 'drop' is called, mmeory is freed

fn gives_ownership() -> String {
    let some_string = String::from("aa");

    some_string
}

fn transfering_and_returns_ownership(some_string: String) -> String {
    println!("transfering ownership of {some_string}");

    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // goes out of scope, nothing happens
