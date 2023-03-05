fn main() {
    for i in 1926..2027 {
        print!("西暦{}年 = ", i);
        if i < 1989 {
            println!("昭和{}年", i - 1926 + 1);
        } else if (1989 <= i) && (2019 > i) {
            println!("平成{}年", i - 1989 + 1);
        } else {
            println!("令和{}年", i - 2019 + 1);
        }
    }
}
