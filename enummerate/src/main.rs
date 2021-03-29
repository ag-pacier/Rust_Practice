enum IpAddr {
    V4(String),
    V6(String),
}
//enum is a custom data type like String
//It differs from struct in that the data type doesn't always need to be the same

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    //Starting on section 6.1

    //let home = IpAddr::V4(String::from("127.0.0.1"));

    //let loopback = IpAddr::V6(String::from("::1"));

}
