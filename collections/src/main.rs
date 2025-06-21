fn main() {
    let v: Vec<i32> = Vec::new();


    let mut v = vec![1,2,3];

    v.push(1);

    let third= &v[2];
    // v.push(4); // cant do that, adding new element to vector and memory memory in heap and we will try to use reference in line below to deallocated place.
    println!("third el {third}");
    let third = v.get(2);

    match third {
        Some(third) => println!("the third el is {third}"),
        None => println!("there is no thrid element.")
    }


    // let does_not_exist = &v[100]; this will cause program to crash, line below will get none value
    let does_not_exist = v.get(100);

    let mut string = String::from("fsd");
    let copy = &string;

    // string =  String::from("fsd");

    // print!("string copy {copy}")

    let mut v = vec![100, 500, 1200];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i +=50;
    }
     for i in &mut v {
        *i +=50;
    }

     enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("fds"))
    ];

}
