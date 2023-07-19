use std::collections::BTreeMap;



pub struct BuildingsList {
    pub buildings: BTreeMap<u32, Building>
}

pub fn create_buildings_list(
    mut input_buildings: Vec<Building>
) -> BuildingsList {
    let mut buildings: BTreeMap<u32, Building> = BTreeMap::new();

    for id in 0..input_buildings.len() {
        buildings.insert(id as u32, input_buildings.remove(0));
    }

    BuildingsList {
        buildings: buildings
    }
}


#[derive(Debug)]
pub enum BuildingType {
    TakeBox (u32), // output id
    GiveBox (u32), // input id
    Factory ((u32, u32), (u32, u32), (u32, u32)) // in(id, amount), out(id, amount), inventory(in quantity, out quantity)
}

#[derive(Debug)]
pub struct Building {
    pub pos: [u32; 2],
    pub building_type: BuildingType,
}


impl Building {
    pub fn take_from(
        &mut self,
        item_id: &u32,
        amount: &u32
    ) -> bool {
        match self.building_type {
            BuildingType::TakeBox(output_id) => {
                if output_id == *item_id {
                    return true;
                }
                return false;
            }
            BuildingType::Factory(_, output, ref mut inventory) => {
                if output.0 == *item_id && inventory.1 >= *amount{
                    inventory.1 -= *amount;
                    return true;
                }
                return false;
            }
            _ => {return false;}
        }
    }

    pub fn give_to(
        &mut self,
        item_id: &u32,
        amount: &u32
    ) -> bool {
        match self.building_type {
            BuildingType::GiveBox(input_id) => {
                if input_id == *item_id {
                    return true;
                }
                return false;
            }
            BuildingType::Factory(input, output, ref mut inventory) => {
                if input.0 == *item_id{
                    inventory.0 += *amount;
                    // process materials
                    while inventory.0 >= input.1 {
                        inventory.1 += output.1;
                        inventory.0 -= input.1;
                    }
                    return true;
                }
                return false;
            }
            _ => {return false;}
        }
    }
}