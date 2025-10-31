use std::collections::HashMap;

use crate::{task_1_1::{countdown, day_of_week, max_of_three, sum_even_numbers}, task_1_2::{concat_strings, factorial, find_max, is_prime, reverse_string}, task_1_3::{Student, TrafficLight, safe_divide}, task_1_4::{filter_even_numbers, print_population, square_elements, sum_odd_numbers}, task_1_5::{parse_integer, read_file_lines, write_file}};

mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

fn main() {
    
     //Testing for tasks in 1
    
    max_of_three(10, 20, 15); 
    max_of_three(10, 9, 10); 
    sum_even_numbers(); 
    countdown();
    println!("{}",day_of_week(4)); 
    println!("{}",day_of_week(8)); 


     //Testing for tasks in 2

    println!("{}", factorial(5)); 

    println!("{}", is_prime(13));

    // Reverse String
    let mut s = String::from("Test"); 
    reverse_string(&mut s);
    println!("{:?}", s);

    // Concatenation 
    let mut t = String::from("lauf"); 
    let concatenated = concat_strings(&s, &t); 
    println!("{:?}", concatenated);


    // Slice-Max Value 
    let numbers = [3, 7, 2, 9, 5];
    match find_max(&numbers) {
        Some(max) => println!("The maximum value is {}", max),
        None => println!("The slice is empty"),
    } 


   // Testing for Tasks in 3

   // Traffic Light
   let student1 : Student = Student { name: (String::from("abc")), age: (1), gpa: (5.0) }; 
   student1.display();

   let traf : TrafficLight = TrafficLight::Green; 
   println!("{}", traf.light_duration());

   //Save-Divide
   println!("{:?}", safe_divide(10, 3)); 
   println!("{:?}", safe_divide(10, 0)); 

    

    // Testing Task 4

    // Vector-Squaring
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    let squared = square_elements(&v);  
    println!("{:?}", squared); 


    //HashMap-City
    let mut city_population: HashMap<String, i32> = HashMap::new();

   
    city_population.insert(String::from("Paris"), 2148000);
    city_population.insert(String::from("Berlin"), 3769000);
    city_population.insert(String::from("Tokyo"), 13929000);
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_population(&city_population, "Paris");
    print_population(&city_population, "London");


    //Filter
    let evens = filter_even_numbers(&numbers);
    println!("Original numbers: {:?}", numbers);
    println!("Even numbers: {:?}", evens); 

    //Sum 
    let sum_odds = sum_odd_numbers(&numbers);
    println!("Sum of odd numbers: {}", sum_odds);
    

    // Testing Task 5 
    



    // Parse_integer
    
    let inputs = vec!["42", "-7", "abc", "3.14"];
    for s in inputs {
        match parse_integer(s) {
            Ok(num) => println!("Parsed '{}' successfully: {}", s, num),
            Err(err) => println!("Error parsing '{}': {}", s, err),
        }
    }

    // Write_file
    
    let test_file = "test_output.txt";
    let content = "Hello, Rust!\nThis is a test.";
    match write_file(test_file, content) {
        Ok(()) => println!("Successfully wrote to '{}'", test_file),
        Err(err) => println!("Failed to write file '{}': {}", test_file, err),
    }

    //  read_file_lines
    
    match read_file_lines(test_file) {
        Ok(lines) => {
            println!("Contents of '{}':", test_file);
            for line in lines {
                println!("{}", line);
            }
        }
        Err(err) => println!("Failed to read file '{}': {}", test_file, err),
    }
 





}
