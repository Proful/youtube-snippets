#![allow(warnings)]

#[derive(Debug, PartialEq)]
struct Employee {
    name: String,
}

impl Employee {
    fn is_same_name(&self, other: &Employee) -> bool {
        if self.name == other.name {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Employee;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // cargo test another
    #[test]
    fn another() {
        assert_eq!(2 + 2, 3);
        // panic!("Failed");
    }
    #[test]
    fn same_emp() {
        let emp1 = Employee {
            name: "Proful".to_string(),
        };
        let emp2 = Employee {
            name: "Proful".to_string(),
        };
        assert!(emp1.is_same_name(&emp2));
        assert_eq!(emp1, emp2);
    }
    #[test]
    fn different_emp() {
        let emp1 = Employee {
            name: "Proful".to_string(),
        };
        let emp2 = Employee {
            name: "Kenny".to_string(),
        };
        assert!(!emp1.is_same_name(&emp2));
        // uses == so value to be compared must implement PartialEq & Debug
        assert_eq!(emp1.is_same_name(&emp2), false);
        assert_eq!(emp1, emp2);
    }
}
