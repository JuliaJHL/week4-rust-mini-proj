fn main() {
    // load dictionary
    let dictionary = spell_corrector::read_txt("count_1w.txt");
    // read word from stdin
    println!("Enter a word:");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    word = word.trim().to_string().to_lowercase();
    // check if word is in dictionary
    if dictionary.contains(&word) {
        println!("Correct spelling.");
        return;
    } else {
        let closest = spell_corrector::closest(&word, dictionary);
        println!("Did you mean {}?", closest);
    }
}
