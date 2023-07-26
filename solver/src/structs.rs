#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Student {
    pub id: usize,
    pub name: String,
    pub academic_ability: usize,
    pub exercise_ability: usize,
    pub leadership_ability: usize,
    pub needs_assistance: bool,
    pub gender: Gender,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    Male,
    Female,
}

pub type SeatAssignment = Vec<Vec<usize>>;