pub mod advent {
    use std::fs;
    use std::error::Error;

    pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(input)?;
        let mut three = 0;
        let mut two = 0;

        for line in content.lines() {
            let mut th = false;
            let mut tw = false;

            for c in line.chars() {
                let count: Vec<&str> = line.matches(c).collect();
                if count.len() == 3 {
                    th = true;
                } else if count.len() == 2 {
                    tw = true;
                }
            }

            if th {
                three += 1;
            }

            if tw {
                two += 1;
            }
        }

        println!("Part1: Two: {}; Three: {}; Result: {}", two, three, two * three);

        let mut w1: Option<&str> = None;
        let mut w2: Option<&str> = None;

        let words: Vec<&str> = content.lines().collect();
        'outer: for (idx, word) in words.iter().enumerate() {
            for other_word in words.iter().skip(idx+1) {
                if word.len() != other_word.len() {
                    continue;
                }

                let mut found = true;
                let mut n_diff = 0;
                for (c1, c2) in word.chars().zip(other_word.chars()) {
                    if c1 != c2 {
                        n_diff += 1;
                    }

                    if n_diff > 2 {
                        found = false;
                        break;
                    }
                }

                if found {
                    w1 = Some(word);
                    w2 = Some(other_word);
                    break 'outer;
                }

            }
        }

        if w1.is_none() || w2.is_none() {
            return Err("Oops, box not found!".into());
        }

        let out: String = w1.unwrap().chars().zip(w2.unwrap().chars())  // iterate over all chars
                    .filter(|(c1, c2)| c1 == c2)                        // if they are the same we keep them
                    .map(|(c, _)| c)                                    // we need this because our filter return (char, char)
                    .collect();                                         // we collect only the first char (or second) of the tuple and make a string

        println!("Part 2: {} <=> {} = {}", w1.unwrap(), w2.unwrap(), out);

        Ok(())
    }
}