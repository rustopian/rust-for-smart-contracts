fn main() {
    let red_potion = create_potion("Red", 10);
    let blue_potion = create_potion(String::from("Blue"), 20);
    let mixed_potion = mix_potions(red_potion, "Green 15");

    display_potion(mixed_potion);
}

fn create_potion(color: String, strength: i32) -> (String, i32) {
    println!("Creating a {} potion with strength {}.", color, strength);
    (color, strength)
}

fn mix_potions(potion1: (String, i32), potion2: (String, i32)) -> (String, i32) {
    let mixed_color = potion1.0 + " and " + &potion2.0;
    let mixed_strength = potion1.1 + potion2.2;
    (mixed_color, mixed_strength)
}

fn display_potion(potion: (String, i32)) {
    println!("Potion Color: {}", potion.0);
    println!("Potion Strength: {}", potion.1);
}