struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn test_panic() {
        panic!("Test panic")
    }

    fn square(side: i32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, cmp_rect: &Self) -> bool {
        self.width > cmp_rect.width && self.height > cmp_rect.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", &name[1..])
}

#[cfg(test)]
mod general_tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(4, add(2, 2));
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn check_panic() {
        Rectangle::test_panic()
    }

    #[test]
    fn another() {
        panic!("Test fail");
    }

    #[test]
    fn greeting_works() {
        let greeting_value = greeting("John");

        assert!(
            greeting_value.contains("John"),
            "'greeting' does returned value with 'John' in it, instead: {}",
            greeting_value
        );
    }

    #[test]
    fn sum_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 20,
        };

        let smaller = Rectangle {
            width: 1,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 20,
        };

        let smaller = Rectangle {
            width: 1,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
