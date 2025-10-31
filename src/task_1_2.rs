use core::slice;

// Function: factorial (recursive)
pub fn factorial(n: u32) -> u32 {
    if n == 1 || n == 0{ 
        1

    } else {
        n * factorial(n-1) 
    }

}

// Function: is_prime (using factorial function)
pub fn is_prime(n: u32) -> bool {
    if n<= 1{
        return false; 
    }

    /*  By Wilson's Theorem, a number is prime iff (n-1)! equiv -1(mod n)
     works only with n<14 else overflow */
    let fact = factorial(n-1); 
    (fact+1)% n == 0
}

// Ownership and Borrowing

// Function: reverse_string (mutably borrow and reverse in place)
pub fn reverse_string(s: &mut String) {
    // This works only with ASCII-Character Strings, in-place string-reversion with UTF-8 characters is trickier (maybe not even possible?) 

    unsafe{ 
    
        let bytes = s.as_bytes_mut();  
        
        let mut head = 0; 
        let mut tail = bytes.len()-1; 

        while head < tail {
            bytes.swap(head, tail);

            head += 1; 
            tail -= 1; 
            
        }
    }

    



}

// Function: concat_strings (concatenates two &str and returns a String)
pub fn concat_strings(s1: &str, s2: &str) -> String {
     format!("{s1}{s2}")  

}

// Slices

// Function: find_max (finds the maximum value in a slice of integers)
pub fn find_max(slice: &[i32]) -> Option<i32> {
    if slice.is_empty(){
        return None;
    }

    let mut max = slice[0]; 
    for &item in slice.iter(){
        if item > max {
            max = item; 
        }
    }
    Some(max) 
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(13));
        assert!(!is_prime(1));
        assert!(!is_prime(10));
    }

    #[test]
    fn test_reverse_string() {
        let mut s = String::from("abcd");
        reverse_string(&mut s);
        assert_eq!(s, "dcba");

        let mut s2 = String::from("racecar");
        reverse_string(&mut s2);
        assert_eq!(s2, "racecar");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("Hello", "World");
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_find_max() {
        let nums = [3, 7, 2, 9, 5];
        assert_eq!(find_max(&nums), Some(9));

        let empty: [i32; 0] = [];
        assert_eq!(find_max(&empty), None);
    }
}
