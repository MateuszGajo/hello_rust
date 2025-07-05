fn main() {
    println!("Hello, world!");
    iterator_method();
    iterator_sum();
    iterator_producing_other();
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}


fn iterator_method() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);
}


fn iterator_sum() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let total:i32 = v1_iter.sum();

    assert_eq!(total, 6)
}

fn iterator_producing_other() {
    let v1: Vec<i32> = vec![1,2,3];

    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    assert_eq!(v2, vec![2,3,4])
}