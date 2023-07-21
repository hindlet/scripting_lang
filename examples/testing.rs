use scripting_lang::*;


fn main() {
    
    let items = create_item_list(vec!["Ore", "Bar"]);

    let mut buildings = create_buildings_list(vec![
        Building {
            pos: [5, 5],
            building_type: BuildingType::TakeBox(0)
        },
        Building {
            pos: [5, 0],
            building_type: BuildingType::Factory((0, 2), (1, 1), (0, 0))
        },
    ]);

    let builder = file_to_script("assets/script.gobbledygook");

    let script = builder.bind_and_build(
        vec![("mine", 0), ("factory", 1)],
        vec![("raw", 0), ("processed", 1)]
    );

    let mut player = Player::new([0, 5]);

    script.run_n(2, &mut player, &mut buildings, &items);
           


}