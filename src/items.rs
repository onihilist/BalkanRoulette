
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
}