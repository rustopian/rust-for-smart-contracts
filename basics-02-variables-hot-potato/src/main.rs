fn main() {
    let potato = String::from("🥔");
    println!("Game starts with the hot potato: {}", potato);

    // Player 1 picks up the potato (ownership moves)
    let player1 = potato;
    print_potato(player1);

    assert_player_has_potato("Player 1", &player1);

    // Player 1 passes the potato to Player 2 (ownership moves)
    let player2 = player1;
    assert_player_has_potato("Player 2", &player2);

    // this will error! We can't use player1 as value has moved
    assert_player_has_potato("Player 1", &player1);

    // now, player2 wants to cheat... to keep a copy of the potato
    // for himself.
    let player3 = player2;
    // how would you change the above line so the program still compiles?
    assert_player_has_potato("Player 2", &player2);
    assert_player_has_potato("Player 3", &player3);


    // Now, uncomment the line below. Player 1 wants the potato back. How
    // do you give it to them?

    // assert_player_has_potato("Player 1", &player1);
}

fn assert_player_has_potato(player_name: String, potato: &str) {
    println!("{} has the potato!");
}