use std::collections::BTreeMap;

pub struct Player {
    pub pos: [u32; 2],
    pub target: u32,
    pub inventory: BTreeMap<u32, u32>
}

impl Player {
    pub fn new(
        start_pos: [u32; 2],
    ) -> Player {
        Player {
            pos: start_pos,
            target: 0,
            inventory: BTreeMap::new(),
        }
    }

    pub fn add_item(
        &mut self,

        item_id: &u32,
        amount: &u32,
    ) {
        if let Some(current_amount) = self.inventory.get_mut(item_id) {
            *current_amount += amount;
        } else {
            self.inventory.insert(*item_id, *amount);
        }
    }

    pub fn take_item(
        &mut self,

        item_id: &u32,
        amount: &u32,
    ) -> bool {
        if let Some(current_amount) = self.inventory.get_mut(item_id) {
            if let Some(new_amount) = current_amount.checked_sub(*amount) {
                *current_amount = new_amount;
                return true;
            }
        }
        false
    }
}