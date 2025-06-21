#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
}


struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    fn addr_v4(address: String) -> Self {
        Self {
            kind: IpAddrKind::V4(address)
        }
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    println!("Hello, world!");

    let four  = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6;

    println!("{:?}", four);

    let address_v4 = IpAddr::addr_v4(String::from("1.1.1.1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = Option::None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;

    let sum = match y {
        Some(value) => x+ value,
        None => x
    };
    println!("sum {sum}")
}
