use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let sentence = vec!["Hello", "Cruel", "World", "!!!"];
    let mut hasher = DefaultHasher::new();
    for word in &sentence {
        print!("{} ", word);
    }

    println!("\n{:?}", sentence);

    let mapped_vals: Vec<_> = sentence
        .into_iter()
        .map(|word| {
            word.hash(&mut hasher);
            return hasher.finish();
        })
        .collect();

    println!("\n{:?}", mapped_vals);

    // Only print odd numbers
    for n in 0..10 {
        if n % 2 == 0 {
            continue;
        }
        println!("{}", n);
    }

    let numbers = [ 3, 59, 97, 729, 17031158983 ];

    for number in numbers {
        println!("is_prime({}): {}", number, is_prime(number));
    }

}


fn is_prime(n: i64) -> bool {
    if n==1 || n == 2 {
        return true;
    }

    for i in 2..n/2 {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}
