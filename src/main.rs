use cli_table::{print_stdout, Table, WithTitle};
use fxhash::{FxHashMap, FxHashSet};
use strum_macros::Display;

use rayon::prelude::*;
use std::{iter::Copied, slice::Iter};

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash, Display)]
enum House {
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash, Display)]
enum Nation {
    British,
    Danish,
    German,
    Norwegian,
    Swedish,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash, Display)]
enum Drink {
    Beer,
    Coffee,
    Milk,
    Tea,
    Water,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash, Display)]
enum Cigar {
    Master,
    Dunhill,
    Pall,
    Prince,
    Blend,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash, Display)]
enum Pet {
    Cat,
    Bird,
    Dog,
    Fish,
    Horse,
}

#[derive(Debug, Clone, Copy, Table)]
struct Entity {
    #[table(title = "House")]
    house: House,
    #[table(title = "Nation")]
    nation: Nation,
    #[table(title = "Drink")]
    drink: Drink,
    #[table(title = "Cigar")]
    cigar: Cigar,
    #[table(title = "Pet")]
    pet: Pet,
}

impl Cigar {
    fn iter() -> Copied<Iter<'static, Cigar>> {
        [
            Cigar::Blend,
            Cigar::Dunhill,
            Cigar::Master,
            Cigar::Pall,
            Cigar::Prince,
        ]
        .iter()
        .copied()
    }
}

impl Pet {
    pub fn iter() -> Copied<Iter<'static, Pet>> {
        [Pet::Bird, Pet::Cat, Pet::Dog, Pet::Fish, Pet::Horse]
            .iter()
            .copied()
    }
}

impl House {
    pub fn iter() -> Copied<Iter<'static, House>> {
        [
            House::Blue,
            House::Green,
            House::Red,
            House::White,
            House::Yellow,
        ]
        .iter()
        .copied()
    }
}

impl Nation {
    pub fn iter() -> Copied<Iter<'static, Nation>> {
        [
            Nation::British,
            Nation::Danish,
            Nation::German,
            Nation::Norwegian,
            Nation::Swedish,
        ]
        .iter()
        .copied()
    }
}

impl Drink {
    pub fn iter() -> Copied<Iter<'static, Drink>> {
        [
            Drink::Beer,
            Drink::Coffee,
            Drink::Milk,
            Drink::Tea,
            Drink::Water,
        ]
        .iter()
        .copied()
    }
}

// 1. The Englishman lives in the red house.
// 2. The Swede keeps dogs.
// 3. The Dane drinks tea.
// 4. The green house is just to the left of the white one.
// 5. The owner of the green house drinks coffee.
// 6. The Pall Mall smoker keeps birds.
// 7. The owner of the yellow house smokes Dunhills.
// 8. The man in the center house drinks milk.
// 9. The Norwegian lives in the first house.
// 10. The Blend smoker has a neighbor who keeps cats.
// 11. The man who smokes Blue Masters drinks bier.
// 12. The man who keeps horses lives next to the Dunhill smoker.
// 13. The German smokes Prince.
// 14. The Norwegian lives next to the blue house.
// 15. The Blend smoker has a neighbor who drinks water.
#[derive(PartialEq, Eq, Hash, Debug)]
enum Rule {
    Rule1,
    Rule2,
    Rule3,
    Rule4,
    Rule5,
    Rule6,
    Rule7,
    Rule8,
    Rule9,
    Rule10,
    Rule11,
    Rule12,
    Rule13,
    Rule14,
    Rule15,
}

struct Constraint {
    results: FxHashMap<Rule, bool>,
}

impl Constraint {
    fn default() -> Self {
        Constraint {
            results: FxHashMap::default(),
        }
    }

    // fn check(&self) -> bool {
    //     self.results.iter().map(|(_, v)| v).all(|v| *v)
    // }

    fn all_passed(&self) -> bool {
        let count = self.results.iter().map(|(_, v)| v).filter(|&&r| r).count();
        count == 15
    }
    fn satisfies_ent(&mut self, item: &Entity) -> bool {
        match item.nation {
            Nation::British => {
                if House::Red == item.house {
                    self.results.insert(Rule::Rule1, true);
                } else {
                    self.results.insert(Rule::Rule1, false);
                    return false;
                }
            }
            Nation::Swedish => {
                if Pet::Dog == item.pet {
                    self.results.insert(Rule::Rule2, true);
                } else {
                    self.results.insert(Rule::Rule2, false);
                    return false;
                }
            }
            Nation::Danish => {
                if Drink::Tea == item.drink {
                    self.results.insert(Rule::Rule3, true);
                } else {
                    self.results.insert(Rule::Rule3, false);
                    return false;
                }
            }
            _ => {}
        }
        match item.house {
            House::Green => {
                if Drink::Coffee == item.drink {
                    self.results.insert(Rule::Rule5, true);
                } else {
                    self.results.insert(Rule::Rule5, false);
                    return false;
                }
            }
            House::Yellow => {
                if Cigar::Dunhill == item.cigar {
                    self.results.insert(Rule::Rule7, true);
                } else {
                    self.results.insert(Rule::Rule7, false);
                    return false;
                }
            }
            _ => {}
        }
        match item.cigar {
            Cigar::Pall => {
                if Pet::Bird == item.pet {
                    self.results.insert(Rule::Rule6, true);
                } else {
                    self.results.insert(Rule::Rule6, false);
                    return false;
                }
            }
            Cigar::Master => {
                if Drink::Beer == item.drink {
                    self.results.insert(Rule::Rule11, true);
                } else {
                    self.results.insert(Rule::Rule11, false);
                    return false;
                }
            }
            Cigar::Prince => {
                if Nation::German == item.nation {
                    self.results.insert(Rule::Rule13, true);
                } else {
                    self.results.insert(Rule::Rule13, false);
                    return false;
                }
            }
            _ => {}
        }

        true
    }

    fn satisfies(&mut self, entities: &[Entity]) -> bool {
        if let Some(e) = entities.get(0) {
            if e.nation == Nation::Norwegian {
                self.results.insert(Rule::Rule9, true);
            } else {
                self.results.insert(Rule::Rule9, false);
                return false;
            }
        }

        if let Some(e) = entities.get(2) {
            if e.drink == Drink::Milk {
                self.results.insert(Rule::Rule8, true);
            } else {
                self.results.insert(Rule::Rule8, false);
                return false;
            }
        }

        for pair in entities.windows(2) {
            if !self.satisfies_ent(&pair[0]) {
                return false;
            }
            if House::Green == pair[0].house {
                if House::White == pair[1].house {
                    self.results.insert(Rule::Rule4, true);
                } else {
                    self.results.insert(Rule::Rule4, false);
                    return false;
                }
            }

            if Pet::Cat == pair[0].pet {
                if pair[1].cigar == Cigar::Blend {
                    self.results.insert(Rule::Rule10, true);
                } else {
                    self.results.insert(Rule::Rule10, false);
                    return false;
                }
            } else if Pet::Horse == pair[0].pet {
                if Cigar::Dunhill == pair[1].cigar {
                    self.results.insert(Rule::Rule12, true);
                } else {
                    self.results.entry(Rule::Rule12).or_insert(false);
                    if Some(&false) == self.results.get(&Rule::Rule12) {
                        return false;
                    }
                }
            }

            if Cigar::Dunhill == pair[0].cigar {
                if Pet::Horse == pair[1].pet {
                    self.results.insert(Rule::Rule12, true);
                } else {
                    self.results.entry(Rule::Rule12).or_insert(false);
                    if Some(&false) == self.results.get(&Rule::Rule12) {
                        return false;
                    }
                }
            }

            if Cigar::Blend == pair[0].cigar {
                if Drink::Water == pair[1].drink {
                    self.results.insert(Rule::Rule15, true);
                } else {
                    self.results.entry(Rule::Rule15).or_insert(false);
                    if Some(&false) == self.results.get(&Rule::Rule15) {
                        return false;
                    }
                }
            }

            if Nation::Norwegian == pair[0].nation {
                if House::Blue == pair[1].house {
                    self.results.insert(Rule::Rule14, true);
                } else {
                    self.results.insert(Rule::Rule14, false);
                    return false;
                }
            }

            if Drink::Water == pair[0].drink {
                if Cigar::Blend == pair[1].cigar {
                    self.results.insert(Rule::Rule15, true);
                } else {
                    self.results.entry(Rule::Rule15).or_insert(false);
                    if Some(&false) == self.results.get(&Rule::Rule15) {
                        return false;
                    }
                }
            }
        }
        if let Some(item) = entities.last() {
            if !self.satisfies_ent(item) {
                return false;
            }
        }

        true
    }
}

fn assign_ent(entities: &[Entity]) -> Vec<Entity> {
    let mut dis_cigar: FxHashSet<Cigar> = FxHashSet::default();
    let mut dis_drink: FxHashSet<Drink> = FxHashSet::default();
    let mut dis_house: FxHashSet<House> = FxHashSet::default();
    let mut dis_nation: FxHashSet<Nation> = FxHashSet::default();
    let mut dis_pet: FxHashSet<Pet> = FxHashSet::default();
    for item in entities.iter() {
        dis_cigar.insert(item.cigar);
        dis_drink.insert(item.drink);
        dis_house.insert(item.house);
        dis_nation.insert(item.nation);
        dis_pet.insert(item.pet);
    }
    let mut candidates = vec![];
    for cigar in Cigar::iter() {
        if dis_cigar.contains(&cigar) {
            continue;
        }

        for drink in Drink::iter() {
            if dis_drink.contains(&drink) {
                continue;
            }
            for house in House::iter() {
                if dis_house.contains(&house) {
                    continue;
                }
                for nation in Nation::iter() {
                    if dis_nation.contains(&nation) {
                        continue;
                    }
                    for pet in Pet::iter() {
                        if dis_pet.contains(&pet) {
                            continue;
                        }

                        let local_ent = Entity {
                            cigar,
                            drink,
                            house,
                            nation,
                            pet,
                        };

                        let mut constrain = Constraint::default();
                        let mut temp_ents = entities.to_vec();
                        temp_ents.push(local_ent);

                        if constrain.satisfies(&temp_ents) {
                            candidates.push(local_ent);
                        }
                    }
                }
            }
        }
    }
    candidates
}

fn backtracking_search(length: usize, confirmed: &Vec<Entity>) -> Option<Vec<Entity>> {
    if length == confirmed.len() {
        let mut constrain = Constraint::default();
        if constrain.satisfies(confirmed) && constrain.all_passed() {
            return Some(confirmed.to_vec());
        } else {
            return None;
        }
    }

    let candidates = assign_ent(confirmed);

    candidates
        .par_iter()
        .map(|ent| {
            let mut next_confirmed = confirmed.clone();
            next_confirmed.push(*ent);
            backtracking_search(length, &next_confirmed)
        })
        .find_first(Option::is_some)
        .unwrap_or(None)
}

fn main() {
    let entities = backtracking_search(5, &vec![]).expect("Failed to solve the puzzle");

    print_stdout(entities.with_title()).unwrap();
}
