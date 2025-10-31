// Variable Binding 

fn variable_binding (){
let integer: i32 = 10;  
let float: f64 = 20.12; 
let bool: bool = true; 
let string: String = String::from("Hello, world!"); 
}




// Conditional Statements: max_of_three function
pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a > b && a > c {
        println!("Max of three is {}", a); 
        a
    } else if b > a && b > c {
        println!("Max of three is {}", b);
        b
    } else if c > a && c > b {
        println!("Max of three is {}", c);
        c
    } else {
        println!("Tie or all equal!");
        0
    }
    
}

// Loops: sum_even_numbers function
pub fn sum_even_numbers() -> i32 {
    let mut sum = 0; 
    for  n in 1..=100  { 
        if n%2 == 0{
            sum+=n; 
        }
    }
    println!("The sum of all even numbers in the range is {sum} "); 
    sum 
    

}

// Loops: Countdown function with while loop
pub fn countdown() {
    let mut index = 10; 
    while index >= 1{
        println!("{}",index);  
        index -= 1; 
    }
    println!("Liftoff"); 
}

// Match Statement: day_of_week function
pub fn day_of_week(day: i32) -> &'static str {
    match day{
        1 => "Monday", 
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Please enter a number in the range 1 to 7",
        
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_of_three() {
        assert_eq!(max_of_three(10, 20, 15), 20);
        assert_eq!(max_of_three(5, 5, 5), 0);
        assert_eq!(max_of_three(-1, -2, -3), -1);
    }

    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(sum_even_numbers(), 2550);
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(day_of_week(1), "Monday");
        assert_eq!(day_of_week(7), "Sunday");
        assert_eq!(day_of_week(9), "Please enter a number in the range 1 to 7");
    }

    #[test]
    fn test_countdown_runs() {
        countdown(); 
    }
}
