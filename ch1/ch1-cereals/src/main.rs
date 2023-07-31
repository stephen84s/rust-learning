#[derive(Debug)]
enum Cereal {
    Barley, Rice, Millet,
    Rye, Spelt, Wheat,

}

impl Clone for Cereal {
    fn clone(&self) -> Cereal {
        match self {
            stuff => 
        }
    }    
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    // drop(grains);

    let y = grains;

    println!("{:?}", y);
}
