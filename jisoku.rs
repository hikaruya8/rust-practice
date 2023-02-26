fn main() {
    let moon = 384400.0;
    let car = 80.0;
    let shinkansen = 300.0;
    println!("自動車がかかる日にち {}日:", moon / car / 24.0);
    println!("新幹線がかかる日にち {}日", moon / shinkansen / 24.0);
}
