use std::collections::HashMap;

 


// Function to square each element in a vector and return a new vector
pub fn square_elements(v: &Vec<i32>) -> Vec<i32> {
    let mut v = v.clone(); 
    for i in &mut v{
        *i = *i * *i; 
    }
    v
}

// HashMap

// Function to print population of a given city or a not found message
pub fn print_population(city_population: &HashMap<String, i32>, city: &str) {
     match city_population.get(city) {
        Some(&pop) => println!("The population of {} is {}", city, pop),
        None => println!("No city found with the name {}", city),
    }
}

// Function to filter even numbers from a vector using an iterator
pub fn filter_even_numbers(v: &Vec<i32>) -> Vec<i32> {
    v.iter().filter(|&x| x%2 == 0).copied().collect()
}

// Function to sum odd numbers in a vector using an iterator
pub fn sum_odd_numbers(v: &Vec<i32>) -> i32 {
    v.iter().filter(|&x| x%2 != 0).sum()
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_square_elements() {
        let v = vec![1, 2, 3, 4];
        let squared = square_elements(&v);
        assert_eq!(squared, vec![1, 4, 9, 16]);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_filter_even_numbers() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let evens = filter_even_numbers(&v);
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn test_sum_odd_numbers() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = sum_odd_numbers(&v);
        assert_eq!(sum, 9); // 1 + 3 + 5 = 9
    }

    #[test]
    fn test_print_population_found() {
        let mut city_population = HashMap::new();
        city_population.insert(String::from("Berlin"), 3769000);
        city_population.insert(String::from("Paris"), 2148000);

        
        print_population(&city_population, "Berlin");
        print_population(&city_population, "Paris");
    }

    #[test]
    fn test_print_population_not_found() {
        let mut city_population = HashMap::new();
        city_population.insert(String::from("Tokyo"), 13929000);

        print_population(&city_population, "London"); 
    }
}
