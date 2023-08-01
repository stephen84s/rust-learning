fn main() {
    let sentence = vec!["Hello", "Cruel", "World", "!!!"];
    for word in &sentence {
        print!("{} ", word);
    }

    // Following line will kill the program
    println!("{}", sentence[0]);
}
