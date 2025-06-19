fn main() {
    const THREE_HOURS_IN_SECONDS: u32= 60*60*3;
   let mut x = 5;
   println!("the value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");


    let y = 5;


    let y = y+1;

    {
        let y = y *2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is {y}");

    let guess: u32 = "42".parse().expect("not a number");

    let tup: (i32, f64, u8) = (500, 6.4,1);

    let (x,y,z) = tup;

    println!("the value of y is {y}");

    let a = [1,2,3,4,5];
    let b = [3;5];

    another_function(5);

    let condition = true;
    let number = if condition {5} else {6};

    println!("Number is {}", number);

    let mut counter =0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter *2;
        }
    };

    println!("result value is {}",result);

    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1
        }
        count +=1
    }
    println!("end count {}", count);

    let arr = [10, 20];

    for element in arr {
        println!(" the value is {}", element);
    }

    for number in (1..4).rev() {
     println!("value is {}", number);
    }

}


fn another_function(x: i32) -> i32{
    println!("You passed {}", x);

      let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    return 2;
}