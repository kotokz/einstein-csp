use std::{
    collections::{HashMap, HashSet},
    iter::Copied,
    slice::Iter,
};

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash)]
enum House {
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash)]
enum Nation {
    British,
    Danish,
    German,
    Norwegian,
    Swedish,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash)]
enum Drink {
    Beer,
    Coffee,
    Milk,
    Tea,
    Water,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash)]
enum Cigar {
    Master,
    Dunhill,
    Pall,
    Prince,
    Blend,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Hash)]
enum Pet {
    Cat,
    Bird,
    Dog,
    Fish,
    Horse,
}

#[derive(Debug, Default, Clone, Copy)]
struct Entity {
    house: Option<House>,
    nation: Option<Nation>,
    drink: Option<Drink>,
    cigar: Option<Cigar>,
    pet: Option<Pet>,
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

struct Contrains {
    results: HashMap<Rule, bool>,
}

impl Contrains {
    fn default() -> Self {
        let mut hashmap: HashMap<Rule, bool> = HashMap::new();
        hashmap.insert(Rule::Rule8, true);
        hashmap.insert(Rule::Rule9, true);
        Contrains { results: hashmap }
    }

    fn check(&self) -> bool {
        let result = self.results.iter().map(|(_, v)| v).all(|v| *v);

        result
    }

    fn all_passed(&self) -> bool {
        let count = self.results.iter().map(|(_, v)| v).filter(|&&r| r).count();
        count == 15
    }
    fn satisfies_ent(&mut self, entities: &[Entity]) -> Option<bool> {
        for item in entities.iter() {
            if let Some(nation) = item.nation {
                match nation {
                    Nation::British => {
                        if Some(House::Red) == item.house {
                            self.results.insert(Rule::Rule1, true);
                        } else {
                            self.results.insert(Rule::Rule1, false);
                        }
                    }
                    Nation::Swedish => {
                        if Some(Pet::Dog) == item.pet {
                            self.results.insert(Rule::Rule2, true);
                        } else {
                            self.results.insert(Rule::Rule2, false);
                        }
                    }
                    Nation::Danish => {
                        if Some(Drink::Tea) == item.drink {
                            self.results.insert(Rule::Rule3, true);
                        } else {
                            self.results.insert(Rule::Rule3, false);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(house) = item.house {
                match house {
                    House::Green => {
                        if Some(Drink::Coffee) == item.drink {
                            self.results.insert(Rule::Rule5, true);
                        } else {
                            self.results.insert(Rule::Rule5, false);
                        }
                    }
                    House::Yellow => {
                        if Some(Cigar::Dunhill) == item.cigar {
                            self.results.insert(Rule::Rule7, true);
                        } else {
                            self.results.insert(Rule::Rule7, false);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(cigar) = item.cigar {
                match cigar {
                    Cigar::Pall => {
                        if Some(Pet::Bird) == item.pet {
                            self.results.insert(Rule::Rule6, true);
                        } else {
                            self.results.insert(Rule::Rule6, false);
                        }
                    }
                    Cigar::Master => {
                        if Some(Drink::Beer) == item.drink {
                            self.results.insert(Rule::Rule12, true);
                        } else {
                            self.results.insert(Rule::Rule12, false);
                        }
                    }
                    Cigar::Prince => {
                        if Some(Nation::German) == item.nation {
                            self.results.insert(Rule::Rule13, true);
                        } else {
                            self.results.insert(Rule::Rule13, false);
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn satisfies_cmp(&mut self, entities: &[Entity]) -> Option<bool> {
        for pair in entities.windows(2) {
            if Some(House::Green) == pair[0].house {
                if let Some(rhouse) = pair[1].house {
                    if rhouse == House::White {
                        self.results.insert(Rule::Rule4, true);
                    } else {
                        self.results.insert(Rule::Rule4, false);
                    }
                }
            }

            if let Some(pet) = pair[0].pet {
                match pet {
                    Pet::Cat => {
                        if let Some(r_cigar) = pair[1].cigar {
                            if r_cigar == Cigar::Blend {
                                self.results.insert(Rule::Rule10, true);
                            } else {
                                self.results.insert(Rule::Rule10, false);
                            }
                        }
                    }
                    Pet::Horse => {
                        if Some(Cigar::Dunhill) == pair[1].cigar {
                            self.results.insert(Rule::Rule11, true);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(cigar) = pair[0].cigar {
                match cigar {
                    Cigar::Dunhill => {
                        if Some(Pet::Horse) == pair[1].pet {
                            self.results.insert(Rule::Rule11, true);
                        }
                    }
                    Cigar::Blend => {
                        if Some(Drink::Water) == pair[1].drink {
                            self.results.insert(Rule::Rule15, true);
                        }
                    }
                    _ => {}
                }
            }

            if Some(Nation::Norwegian) == pair[0].nation {
                if let Some(r_house) = pair[1].house {
                    if r_house == House::Blue {
                        self.results.insert(Rule::Rule14, true);
                    } else {
                        self.results.insert(Rule::Rule14, false);
                    }
                }
            }

            if Some(Drink::Water) == pair[0].drink && Some(Cigar::Blend) == pair[1].cigar {
                self.results.insert(Rule::Rule15, true);
            }
        }

        None
    }
    fn satisfies(&mut self, entities: &[Entity]) -> bool {
        self.satisfies_ent(entities);
        self.satisfies_cmp(entities);
        self.check()
    }
}

fn assign_ent(entity: &Entity, entities: &[Entity]) -> Vec<Entity> {
    let mut dis_cigar: HashSet<Cigar> = HashSet::new();
    let mut dis_drink: HashSet<Drink> = HashSet::new();
    let mut dis_house: HashSet<House> = HashSet::new();
    let mut dis_nation: HashSet<Nation> = HashSet::new();
    let mut dis_pet: HashSet<Pet> = HashSet::new();
    for item in entities.iter() {
        dis_cigar.insert(item.cigar.unwrap());
        dis_drink.insert(item.drink.unwrap());
        dis_house.insert(item.house.unwrap());
        dis_nation.insert(item.nation.unwrap());
        dis_pet.insert(item.pet.unwrap());
    }
    let mut possibiles = vec![];
    for cigar in Cigar::iter() {
        let cigar = match entity.cigar {
            Some(c) => c,
            None => cigar,
        };
        if dis_cigar.contains(&cigar) {
            continue;
        }
        for drink in Drink::iter() {
            if dis_drink.contains(&drink) {
                continue;
            }
            let drink = match entity.drink {
                Some(d) => d,
                None => drink,
            };
            for house in House::iter() {
                if dis_house.contains(&house) {
                    continue;
                }
                let house = match entity.house {
                    Some(h) => h,
                    None => house,
                };
                for nation in Nation::iter() {
                    if dis_nation.contains(&nation) {
                        continue;
                    }
                    let nation = match entity.nation {
                        Some(n) => n,
                        None => nation,
                    };
                    for pet in Pet::iter() {
                        if dis_pet.contains(&pet) {
                            continue;
                        }
                        let pet = match entity.pet {
                            Some(p) => p,
                            None => pet,
                        };

                        let local_ent = Entity {
                            cigar: Some(cigar),
                            drink: Some(drink),
                            house: Some(house),
                            nation: Some(nation),
                            pet: Some(pet),
                        };

                        let mut constrain = Contrains::default();
                        let mut temp_ents = entities.to_vec();
                        temp_ents.push(local_ent);

                        if constrain.satisfies(&temp_ents) {
                            possibiles.push(local_ent);
                        }
                    }
                }
            }
        }
    }
    possibiles
}

fn backtracking_search(
    ents: &[Entity],
    confirmed: &Vec<Entity>,
    candidates: &[Entity],
) -> Option<Vec<Entity>> {
    if ents.len() == confirmed.len() {
        let mut constrain = Contrains::default();
        if constrain.satisfies(confirmed) && constrain.all_passed() {
            return Some(confirmed.to_vec());
        } else {
            return None;
        }
    }

    let mut new_candidates = vec![];
    let mut possibles = vec![];

    for (index, item) in candidates.iter().enumerate() {
        if index == 0 {
            possibles = assign_ent(item, confirmed);
        } else {
            new_candidates.push(*item);
        }
    }

    for ent in possibles.iter() {
        // find a possible entity candidate
        let mut next_confirmed = confirmed.clone();
        next_confirmed.push(*ent);
        if let Some(result) = backtracking_search(ents, &next_confirmed, &new_candidates) {
            return Some(result);
        }
    }
    None
}

fn search(ents: &[Entity]) -> Option<Vec<Entity>> {
    let mut candidates = vec![];
    for ent in ents.iter() {
        candidates.push(*ent);
    }
    backtracking_search(ents, &vec![], &candidates)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut entities = [Entity::default(); 5];
    if let Some(first) = entities.get_mut(0) {
        first.nation = Some(Nation::Norwegian);
    }
    if let Some(middle) = entities.get_mut(2) {
        middle.drink = Some(Drink::Milk);
    }

    let entities = search(&entities).ok_or("Failed to solve the riddle")?;

    for ent in entities.iter() {
        println!(
            "{:?}, {:?}, {:?}, {:?}, {:?}",
            ent.house.unwrap(),
            ent.nation.unwrap(),
            ent.drink.unwrap(),
            ent.cigar.unwrap(),
            ent.pet.unwrap()
        );
    }

    Ok(())
}
