#![allow(warnings)] // NOT RECOMMENDED

struct Employee {
    name: String,
    role: String,
    language: String,
}

trait Person {
    fn my_name(&self) -> &str;
    fn my_role(&self) -> String;
}

trait Programmer: Person {
    fn my_language(&self) -> &str;
    fn my_role(&self) -> String;
}

impl Person for Employee {
    fn my_name(&self) -> &str {
        &self.name
    }

    fn my_role(&self) -> String {
        format!("Mr {}", self.role)
    }
}

impl Programmer for Employee {
    fn my_language(&self) -> &str {
        &self.language
    }

    fn my_role(&self) -> String {
        format!("Ms {}", self.role)
    }
}

fn print_employee_info(emp: &impl Programmer) {
    println!("{} who codes in {}", emp.my_name(), emp.my_language());
}

fn main() {
    let emp = Employee {
        name: "John".to_string(),
        role: "Programmer".to_string(),
        language: "Rust".to_string(),
    };

    print_employee_info(&emp);

    let my_role1 = <Employee as Person>::my_role(&emp);
    let my_role2 = <Employee as Programmer>::my_role(&emp);

    println!("{}", my_role1);
    println!("{}", my_role2);
}
