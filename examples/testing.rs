use scripting_lang::*;


fn main() {
    
    let items = create_item_list(vec!["Ore", "Bar", "Stone", "Concrete"]);

    let mut buildings = create_buildings_list(vec![
        Building {
            pos: [5, 5],
            building_type: BuildingType::TakeBox(0)
        },
        Building {
            pos: [5, 0],
            building_type: BuildingType::Factory((0, 2), (1, 1), (0, 0))
        },
        Building {
            pos: [10, 5],
            building_type: BuildingType::TakeBox(2)
        },
        Building {
            pos: [15, 0],
            building_type: BuildingType::Factory((2, 2), (3, 1), (0, 0))
        },
    ]);

    let builder = file_to_script("assets/script.gobbledygook");

    let script_one = builder.bind_and_build(
        vec![("mine", 0), ("factory", 1)],
        vec![("raw", 0), ("processed", 1)]
    );
    let script_two = builder.bind_and_build(
        vec![("mine", 2), ("factory", 3)],
        vec![("raw", 2), ("processed", 3)]
    );

    let mut player = Player::new([0, 5]);

    script_one.run_n(2, &mut player, &mut buildings, &items);
    script_two.run_n(2, &mut player, &mut buildings, &items);
           


}