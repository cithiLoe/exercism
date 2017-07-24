#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

pub struct Allergies {
    allergies: Vec<Allergen>,
}

impl Allergies {
    pub fn new(sum: u16) -> Self {
        let allergens = vec![Allergen::Eggs,
                             Allergen::Peanuts,
                             Allergen::Shellfish,
                             Allergen::Strawberries,
                             Allergen::Tomatoes,
                             Allergen::Chocolate,
                             Allergen::Pollen,
                             Allergen::Cats];
        let allergies = allergens
            .into_iter()
            .filter(|&a| (a as u16) & sum != 0)
            .collect();

        Allergies { allergies: allergies }
    }

    pub fn allergies(self) -> Vec<Allergen> {
        self.allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }
}
