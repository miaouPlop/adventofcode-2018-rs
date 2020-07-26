pub mod advent {
    use std::fs;
    use std::error::Error;
    use std::collections::HashSet;

    fn get_frequencies(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
        let frequencies: Vec<i32> = fs::read_to_string(input)?
            .lines()
            .map(|line| line.parse::<i32>())
            .filter_map(Result::ok)
            .collect();
    
        Ok(frequencies)
    }
    
    pub fn run(input: Option<&String>) -> Result<(), Box<dyn Error>> {
        if input.is_none() {
            return Err("No input given!".into());
        }
    
        let frequencies: Vec<i32> = get_frequencies(input.unwrap())?;
    
        println!("Freq is: {}", frequencies.iter().sum::<i32>());
    
        let mut known_frequencies = HashSet::new();
        known_frequencies.insert(0);
    
        let mut freq: i32 = 0;
        'outer: loop {
            for f in frequencies.iter() {
                freq += f;
    
                if known_frequencies.contains(&freq) {
                    break 'outer;
                }
    
                known_frequencies.insert(freq);
            }
        }
        
        println!("Freq is: {}", freq);
        
        Ok(())
    }
}
