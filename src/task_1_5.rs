use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
// Function to parse a string into an integer, returns Result type
pub fn parse_integer(s: &str) -> Result<i32, String> {
    match s.parse::<i32>(){
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Failed to parse {}",s)),
    }

    
    }

// File I/O

// Function to read all lines from a file, returns Result type with Vec<String>
pub fn read_file_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, io::Error> {
    // Open the file
    let file = File::open(file_path)?; 

    // Wrap in a buffered reader 
    let reader = io::BufReader::new(file);

    // Collect all lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

// Function to write a string to a file, returns Result type
pub fn write_file<P: AsRef<Path>>(file_path: P, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?; 
    file.write_all(content.as_bytes())?;  
    Ok(())   
}

// Combining Error Handling and File I/O

// Function to read integers from file, compute their sum, and return the result
pub fn read_and_sum_integers<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let file = File::open(&file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result
            .map_err(|e| format!("Failed to read line {}: {}", line_num + 1, e))?;
        let num: i32 = line.trim().parse()
            .map_err(|_| format!("Invalid integer on line {}: '{}'", line_num + 1, line))?;
        sum += num;
    }

    Ok(sum)
}

fn main() {
    let input_path = "numbers.txt";
    let output_path = "sum.txt";

    match read_and_sum_integers(input_path) {
        Ok(sum) => {
            println!("Sum of integers: {}", sum);

            // Write sum to file
            if let Err(err) = write_file(output_path, &sum.to_string()) {
                eprintln!("Failed to write sum to file: {}", err);
            } else {
                println!("Sum written to '{}'", output_path);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("42"), Ok(42));
        assert_eq!(parse_integer("-7"), Ok(-7));
        assert!(parse_integer("abc").is_err());
        assert!(parse_integer("3.14").is_err());
    }

    #[test]
    fn test_write_and_read_file() {
        let test_file = "test_io.txt";
        let content = "Hello\nWorld\n123";

        // Write content
        write_file(test_file, content).expect("Failed to write file");

        // Read content back
        let lines = read_file_lines(test_file).expect("Failed to read file");
        assert_eq!(lines, vec!["Hello", "World", "123"]);

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_read_and_sum_integers() {
        let test_file = "numbers_test.txt";
        let content = "1\n2\n3\n4\n5";

        write_file(test_file, content).expect("Failed to write file");

        let sum = read_and_sum_integers(test_file).expect("Failed to sum integers");
        assert_eq!(sum, 15);

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_read_and_sum_integers_invalid_line() {
        let test_file = "invalid_numbers.txt";
        let content = "1\nabc\n3";

        write_file(test_file, content).expect("Failed to write file");

        let result = read_and_sum_integers(test_file);
        assert!(result.is_err());

        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
