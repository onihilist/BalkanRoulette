<<<<<<< HEAD

pub struct ItemsData {
    items_list: [&'static str;8],
    used_slot: i16,
    empty_slot: i16
}

pub fn init_items_data() -> ItemsData {

    let _items_list: [&str;5] = ["cigarettes", "beer", "magnifying glass", "handcuffs", "saw"];

    ItemsData {
        items_list: ["null", "null", "null", "null", "null", "null", "null", "null"],
        used_slot: 0,
        empty_slot: 8
    }
}

pub fn found_item(item: &str) -> i16 {
    let items_list: [&str;5] = ["cigarettes", "beer", "magnifying glass", "handcuffs", "saw"];
    let mut item_found: i16 = 0;
    let mut counter: i16 = 0;

    while counter < (items_list.len() + 1) as i16 {
        if items_list[counter as usize] == item {
            break;
        } else {
            counter = counter + 1;
        }
    }
    return counter;
=======

pub struct ItemsData {
    items_list: [&'static str;8],
    used_slot: i16,
    empty_slot: i16
}

pub fn init_items_data() -> ItemsData {

    let _ITEMS_LIST: [&str;5] = ["cigarettes", "beer", "magnifying glass", "handcuffs", "saw"];

    ItemsData {
        items_list: ["null", "null", "null", "null", "null", "null", "null", "null"],
        used_slot: 0,
        empty_slot: 8
    }
>>>>>>> 0e34cea7993ea588f7e9cc267844f677be6365f3
}