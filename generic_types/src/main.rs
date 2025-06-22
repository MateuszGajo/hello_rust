fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn points() {
    let integer = Point {x:5, y:10};
    let float = Point {x: 1.0, y: 4.0};
    let float = Point {x: 1.0, y: 4};
}
fn main() {
    let number_list = vec![34,50,100];

    let mut largest_output = largest(&number_list);

   
    println!("The largest number is {largest_output}");

    let number_list: Vec<i64> = vec![60000000000,102, 34,  89, 54, 2, 43, 8];

    let mut largest_output = largest(&number_list);

    println!("The largest number is {largest_output}");
}
