use std::collections::BTreeMap;

use super::*;

#[derive(Debug)]
pub struct Script {
    pub building_bindings: BTreeMap<u32, u32>,
    pub item_bindings: BTreeMap<u32, u32>,
    pub commands: Vec<Command>,
}



impl Script {

    pub fn bind_buildings(&mut self, building_ids: Vec<u32>) {
        for index in 0..building_ids.len() {
            self.building_bindings.insert(index as u32, building_ids[index]);
        }
    }

    pub fn bind_items(&mut self, item_ids: Vec<u32>) {
        for index in 0..item_ids.len() {
            self.item_bindings.insert(index as u32, item_ids[index]);
        }
    }


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

#[derive(Debug)]
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