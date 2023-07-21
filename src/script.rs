use std::collections::BTreeMap;

use super::*;


#[derive(Debug)]
pub struct ScriptBuilder {
    pub building_bindings: BTreeMap<String, u32>,
    pub item_bindings: BTreeMap<String, u32>,
    pub commands: Vec<Command>
}

impl ScriptBuilder {
    pub fn bind_and_build(
        &self,
        in_buildings: Vec<(&str, u32)>,
        in_items: Vec<(&str, u32)>
    ) -> Script {
        let mut buildings: BTreeMap<u32, u32> = BTreeMap::new();
        for building_bind in in_buildings.iter() {
            if let Some(local_bind) = self.building_bindings.get(building_bind.0) {
                buildings.insert(local_bind.clone(), building_bind.1);
            }
        }
        if buildings.len() != self.building_bindings.len() {
            panic!("Wrong quantity of bound buildings: expected {}, found {}", self.building_bindings.len(), buildings.len());
        }
        
        let mut items: BTreeMap<u32, u32> = BTreeMap::new();
        for item_bind in in_items.iter() {
            if let Some(local_bind) = self.item_bindings.get(item_bind.0) {
                items.insert(local_bind.clone(), item_bind.1);
            }
        }
        if items.len() != self.item_bindings.len() {
            panic!("Wrong quantity of bound buildings: expected {}, found {}", self.item_bindings.len(), items.len());
        }


        Script {
            building_bindings: buildings,
            item_bindings: items,
            commands: self.commands.clone()
        }
    }
}

#[derive(Debug)]
pub struct Script {
    pub building_bindings: BTreeMap<u32, u32>, // local, global
    pub item_bindings: BTreeMap<u32, u32>, // local, global
    pub commands: Vec<Command>,
}



impl Script {

    pub fn run_once(&self, player: &mut Player, buildings: &mut BuildingsList, items: &ItemList) {
        for command in self.commands.iter() {
            match command {
                Command::Goto {target_id} => {
                    goto(self.building_bindings.get(target_id).unwrap(), player, buildings)
                }
                Command::Deposit {item_id, amount} => {
                    deposit(self.item_bindings.get(item_id).unwrap(), amount, player, buildings);
                }
                Command::Pickup {item_id, amount} => {
                    pickup(self.item_bindings.get(item_id).unwrap(), amount, player, buildings);
                }
                Command::PrintInventory => {print_inventory(player, items);}
            }
        }
    }

    pub fn run_n(&self, num_runs: u32, player: &mut Player, buildings: &mut BuildingsList, items: &ItemList) {
        for _ in 0..num_runs {
            self.run_once(player, buildings, items);
        }
    }


}

#[derive(Debug, Clone)]
pub enum Command {
    Goto {
        target_id: u32
    },
    Pickup {
        item_id: u32,
        amount: u32
    },
    Deposit {
        item_id: u32,
        amount: u32
    },
    PrintInventory
}

fn goto(
    target_id: &u32,
    player: &mut Player,
    buildings: &BuildingsList
) {
    if let Some(target) = buildings.buildings.get(target_id) {
        player.pos = target.pos;
        player.target = *target_id;
    }
    else {
        panic!("Could not find building with id: {}", target_id)
    }
}

fn pickup(
    item_id: &u32,
    amount: &u32,

    player: &mut Player,

    buildings: &mut BuildingsList,
) -> bool {
    if let Some(target) = buildings.buildings.get_mut(&player.target) {
        if target.take_from(item_id, amount) {
            player.add_item(item_id, amount);
            return true;
        }
        return false;
    } else {
        panic!("Could not find building with id: {}", player.target)
    }
    
}

fn deposit(
    item_id: &u32,
    amount: &u32,

    player: &mut Player,

    buildings: &mut BuildingsList,
) -> bool {
    if let Some(target) = buildings.buildings.get_mut(&player.target) {
        let player_amount = player.inventory.get_mut(item_id).unwrap();
        if amount <= player_amount {
            if target.give_to(item_id, amount) {*player_amount -= amount; return true;}
            return false;
        }
        return false;
    }
    else {
        panic!("Could not find building with id: {}", player.target)
    }
}


fn print_inventory(
    player: &Player,
    items: &ItemList
) {
    println!("The player has:");
    for (id, amount) in player.inventory.iter() {
        println!("  - {} {}s", amount, get_item_data(id, items));
    }
}