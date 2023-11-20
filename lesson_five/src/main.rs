mod math_macros;

fn main() {
    let x = 10;
    let y = 5;

    let result = calculate!(x, "+", y); // 进行加法运算
    println!("{} + {} = {}", x, y, result);

    let result = calculate!(x, "*", y); // 进行乘法运算
    println!("{} * {} = {}", x, y, result);
}
