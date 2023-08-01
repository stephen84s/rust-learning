fn main() {
    let twenty = 20;
    let twenty_one:i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;

    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million = 1_000_000i64;
    println!("{}",one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.015_f32,
        42.014_f32
    ];

    println!("{:02}", forty_twos[2]);
}