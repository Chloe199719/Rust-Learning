enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(u8, u8 ,u8, u8)
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let test = 1
    let six = IpAddrKind::V6;
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

fn route(ip_kind: IpAddrKind) {

}
}