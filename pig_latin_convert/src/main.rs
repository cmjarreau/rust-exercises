// Convert strings to pig latin
//
// The first consonate of each word is moved to the end of the word and "ay" is added
// so, "first" becomes "irst-fay." Words that start with a vowel have "hay" added to the
// end instead ("apple" becomes "apple-hay"). Keep in mind details about UTF-8 encoding!

fn main() {
    let word = String::from("first");

    // maybe an enum that contains these called letter
    let vowels = String::from("aeiou");

    // consonates

    // assess if the first letter is a vowel
    let first_letter = &word[0..1];

    let mut pig = String::new();

    for letter in vowels.chars() {
        if first_letter == letter.to_string() {
            pig = String::from(&word);
            pig.push_str("-hay");
            println!("if");
            break;
        } else {
            println!("else");
            let back = &word[1..word.len()];
            pig = String::from(back);

            pig.push_str("-");
            pig.push_str(first_letter);
            pig.push_str("ay");
            break;
        }
    }
    println!("pig: {}", pig);
}
