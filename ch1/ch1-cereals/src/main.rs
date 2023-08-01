#[derive(Debug)]
enum Cereal {
    Barley,
    Rice,
    Millet,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);

    println!("{:?}", grains);
}
