fn main() {
    let abbv = "rust";

    let word_list: Vec<String> = std::fs::read_to_string("words.txt")
        .expect("Error reading")
        .trim()
        .lines()
        .map(str::to_string)
        .collect();

    word_list
        .iter()
        .filter(|&long| word_contains(abbv, long))
        .filter(|word| word.len() < 8)
        .for_each(|word| {
            println!("{word}");

            if let Ok(result) = std::str::from_utf8(
                std::process::Command::new("cargo")
                    .args(["search", word, "--color", "never"])
                    .output()
                    .expect("Cargo not installed")
                    .stdout
                    .as_ref(),
            ) {
                println!("{result}")
            }
        });
}

fn word_contains(abbv: &str, long: &str) -> bool {
    let mut next = 0;
    let mut long = long;
    abbv.chars()
        .filter(|c| {
            let mut result = false;
            for (i, c_long) in long.get(next..).unwrap_or("").chars().enumerate() {
                if c == &c_long {
                    long = long.get(next..).unwrap_or("");
                    next = i + 1;
                    result = true;
                }
            }
            result
        })
        .collect::<Vec<char>>()
        .len()
        == abbv.len()
}
