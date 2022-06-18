use std::collections::{HashMap, HashSet};
#[derive(Debug)]
struct School<T> {
    students: HashMap<String, T>
}

impl<T> School <T>
where
    T: Ord + Clone,
{
    fn new() -> Self {
        School { students: HashMap::new() }
    }

    fn add(&mut self, grade: T, student: &str) {
        self.students.insert(student.to_string(), grade);
    }


    fn grades(&self) -> Vec<T> {
        let mut grades: Vec<T> = vec![];
        for (_, val) in self.students.iter() {
            grades.push(val.clone())
         }
         grades.sort();
         grades.dedup();
         grades

    }

    fn grade(&self, grade: T) -> Vec<String> {
        let mut name_students: Vec<String> = vec![];
        for (name, val) in self.students.iter() {
            if (*val == grade) {
                name_students.push(name.to_owned())
            }
         }
         name_students.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
         name_students
    }

}

fn main() {
}


#[test]
fn check_initial(){
    let school_initial = School::<u32>::new();
    assert_eq!(school_initial.students,   School { students: HashMap::new() }.students);
}

#[test]
fn check_grade_school() {
    let mut school_initial = School::<u32>::new();
    school_initial.add(3,  "Kai");
    school_initial.add(2,  "Lee");
    school_initial.add(2,  "Sin");

    assert_eq!(school_initial.grades(), vec![2, 3])
}

#[test]
fn check_grade_school_adv_string() {
    let mut school_initial = School::<&str>::new();
    school_initial.add("B",  "Kai");
    school_initial.add("A",  "Lee");
    school_initial.add("A",  "Sin");

    assert_eq!(school_initial.grades(), vec!["A", "B"])
}

#[test]
fn check_student_same_grade() {
    let mut school_initial = School::<u32>::new();
    school_initial.add(4,  "Bob");
    school_initial.add(4,  "Alice");
    school_initial.add(5,  "Tom");

    assert_eq!(school_initial.grade(4), vec!["Alice", "Bob"])
}


#[test]
fn check_student_same_grade_adv_string() {
    let mut school_initial = School::<&str>::new();
    school_initial.add("A",  "Bob");
    school_initial.add("A",  "Alice");
    school_initial.add("B",  "Tom");

    assert_eq!(school_initial.grade("A"), vec!["Alice", "Bob"])
}
