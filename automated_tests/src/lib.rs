pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 4,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn exploration() {
        let result = add(2,2);
        assert_eq!(result, 4)
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };

        let smaller = Rectangle {
            width: 4,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger), "Smaller cannot holder larger!!!!!");
    }

    // #[test]
    // fn another() {
    //     panic!("panic")
    // }
}