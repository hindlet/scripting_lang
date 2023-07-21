use std::collections::BTreeMap;
use std::fs;
use super::*;


pub fn file_to_script(
    path: &str
) -> ScriptBuilder {
    text_to_script(&fs::read_to_string(path).unwrap())
}

pub fn text_to_script(
    text: &str,
) -> ScriptBuilder {
    let (buildings_text, items_text, run_text) = break_into_sections(text);

    let building_bindings: BTreeMap<&str, u32> = process_data_section(&buildings_text);
    let item_bindings: BTreeMap<&str, u32> = process_data_section(&items_text);

    let commands = {
        let commands_string = run_text.split("{").collect::<Vec<&str>>()[1];
        let spaceless_commands = commands_string.replace(" ", "");
        let commands_string_list: Vec<&str> = spaceless_commands.split(";").collect();

        let mut commands = Vec::new();
        for i in 0..commands_string_list.len() - 1 {
            commands.push(process_command(commands_string_list[i], &building_bindings, &item_bindings));
        }
        commands
    };

    let final_bindings = {
        let mut new_building_binds = BTreeMap::new();
        for bind in building_bindings {
            new_building_binds.insert(bind.0.to_string(), bind.1);
        }

        let mut new_item_binds = BTreeMap::new();
        for bind in item_bindings {
            new_item_binds.insert(bind.0.to_string(), bind.1);
        }

        (new_building_binds, new_item_binds)
    };

    ScriptBuilder {
        building_bindings: final_bindings.0,
        item_bindings: final_bindings.1,
        commands: commands
    }
}

fn break_into_sections(
    initial_string: &str
) -> (String, String, String){

    let spaceless = initial_string.replace("\n", "");
    let mut sections: Vec<&str> = spaceless.split("}").collect();
    sections.remove(sections.len() - 1);

    if sections.len() != 3 {panic!("Wrong number of sections in script file, has {} but expected 3", sections.len())}
    let mut has_sections = (-1, -1, -1);
    for i in 0..sections.len() {
        if sections[i].starts_with("buildings") {has_sections.0 = i as i32}
        if sections[i].starts_with("items") {has_sections.1 = i as i32}
        if sections[i].starts_with("run") {has_sections.2 = i as i32}
    }

    if has_sections.0 == -1 || has_sections.1 == -1 || has_sections.2 == -1 {panic!("Script did not contain all required sections: \n- Buildings: {} \n- Items: {} \n- Run: {}", has_sections.0 != -1, has_sections.1 != -1, has_sections.2 != -1)};

    (sections[has_sections.0 as usize].to_string(), sections[has_sections.1 as usize].to_string(), sections[has_sections.2 as usize].to_string())
}


fn process_data_section(
    section: &str
) -> BTreeMap<&str, u32> {
    let mut bindings = BTreeMap::new();
    let name_string = section.split("{").collect::<Vec<&str>>()[1];
    let names: Vec<&str> = name_string.split(";").collect();
    for i in 0..names.len() - 1 {
        bindings.insert(names[i].trim(), i as u32);
    }
    bindings
}

fn process_command(
    in_string: &str,
    building_bindings: &BTreeMap<&str, u32>,
    item_bindings: &BTreeMap<&str, u32>
) -> Command{
    let stripped = in_string.replace(")", "");
    let keyword = stripped.split("(").collect::<Vec<&str>>()[0];
    let args: Vec<&str> = stripped.split("(").collect::<Vec<&str>>()[1].split(",").collect();

    match keyword {
        "goto" => {
            return Command::Goto {
                target_id: building_bindings.get(args[0]).unwrap().clone()
            };
        }
        "pickup" => {
            return Command::Pickup {
                item_id: item_bindings.get(args[0]).unwrap().clone(),
                amount: args[1].parse().unwrap()
            };
        }
        "dropoff" => {
            return Command::Deposit {
                item_id: item_bindings.get(args[0]).unwrap().clone(),
                amount: args[1].parse().unwrap()
            };
        }
        "speak" => {
            return Command::PrintInventory;
        }
        _ => panic!("Could not match keyword: \"{}\"", keyword)
    }

    

}