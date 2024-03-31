fn str_binary(s: &str) -> String {
    let bits: String = s.bytes().map(|c| format!("{:08b} ", c)).collect();
    format!("{:08b} {}", s.len(), bits)
}

#[test]
fn binary() {
    let field_name = "name";
    let field_value = "Joonas Kajava";

    println!("{}", str_binary(field_name));
    println!("{}", str_binary(field_value));
}

#[test]
fn other_bin() {
    let action = 1u8;
    let player_id = 2u8;
    let x = 3u8;
    let y = 4u8;
    let formatted_coordinates = format!("{:08b} {:08b} {:08b} {:08b}", action, player_id, x, y);
    println!("{}", formatted_coordinates);
}
