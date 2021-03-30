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

//The major difference between STRUCTs and ENUMs is that STRUCTs can have multiple values. Like an AND.
//ENUMs are an OR so in the above example, an IpAddress will always be EITHER V4 OR V6. I can't store both in there
//SO: IpAddress::V4(192,168,0,1) but nothing about V6