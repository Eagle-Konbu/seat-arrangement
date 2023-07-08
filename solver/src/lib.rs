pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Input {
    students: Vec<Student>,
    row: usize,
    column: usize,
}

struct Student {
    name: String,
    academic_ability: usize,
    exercise_ability: usize,
    leadership: usize,
    needs_assistance: bool,
    gender: Gender,
}

enum Gender {
    Male,
    Female
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
