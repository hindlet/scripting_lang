use std::collections::BTreeMap;


pub struct ItemList {
    pub items: BTreeMap<u32, Item>
}

pub struct Item {
    pub name: String,
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        Item {
            name: value.to_string()
        }
    }
}


pub fn create_item_list(
    item_data: Vec<&str>
) -> ItemList {
    let mut items: BTreeMap<u32, Item> = BTreeMap::new();

    for id in 0..item_data.len() {
        items.insert(id as u32, item_data[id].into());
    }

    ItemList {
        items: items
    }
}

pub fn get_item_data(
    id: &u32,
    items: &ItemList
) -> String {
    if let Some(item) = items.items.get(id) {
        return item.name.clone();
    } else {
        panic!("Could not get item of id: {}", id)
    }
}
