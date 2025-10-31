// Structs
#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
    pub gpa: f32,
}

impl Student {
    // Function to create a new Student instance
    pub fn new_student(name: String, age: u32, gpa: f32) -> Student {
        Student { name, age, gpa } //returns new student
    }

    // Method to display student information
    pub fn display(&self) {
        println!(
            "Name: {}, Age:{}, GPA:{}",
            self.name, self.age,self.gpa
        ); 
    }
}

// Enums
#[derive(Debug)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // Function to return the duration of each light
    pub fn light_duration(&self) -> u32 {
        match self{
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Red => 50, 

        }
    }
}

// Option Enum
// Function to safely divide two integers
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0{
        None 
    } else {
        Some(a/b)
    }

}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_student() {
        let s = Student::new_student(String::from("Alice"), 20, 3.8);
        assert_eq!(s.name, "Alice");
        assert_eq!(s.age, 20);
        assert_eq!(s.gpa, 3.8);
    }

    #[test]
    fn test_traffic_light_durations() {
        let green = TrafficLight::Green;
        let yellow = TrafficLight::Yellow;
        let red = TrafficLight::Red;

        assert_eq!(green.light_duration(), 30);
        assert_eq!(yellow.light_duration(), 5);
        assert_eq!(red.light_duration(), 50);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(7, 3), Some(2)); // integer division
        assert_eq!(safe_divide(10, 0), None);
    }
}
