// Here, we have a String that represents the "hot potato".
// `player` variables take turns holding the potato.
//
// The function `assert_player_has_potato` is used to check if
// a player has the potato; but it will fail to compile if
// ownership is moved.
fn main() {
    let potato = String::from("🥔");
    println!("Game starts with the hot potato: {}", potato);

    // Player 1 picks up the potato (ownership moves)
    let player1 = potato;
    assert_player_has_potato("Player 1", &player1);

    // Player 1 passes the potato to Player 2 (ownership moves)
    let player2 = player1;
    assert_player_has_potato("Player 2", &player2);

    // this will error! We can't use player1 as value has moved.
    // You can delete or comment out this line to fix it.
    assert_player_has_potato("Player 1", &player1);

    // now, player2 wants to cheat... to keep a copy of the potato
    // for herself. But she can't do that like this...
    let player3 = player2;
    // how would you change the above line so that player 2 can 
    // copy the potato and the program still compiles,
    // WITHOUT modifying the bottom 2 lines?
    assert_player_has_potato("Player 2", &player2);
    assert_player_has_potato("Player 3", &player3);


    // Now, player 1 wants the potato back. How
    // do you give it to them?
    // --GIVE IT TO THEM ON THIS LINE--
    //
    // Uncomment the following line and leave it unchanged:
    // assert_player_has_potato("Player 1", &player1);
}

fn assert_player_has_potato(player_name: &str, potato: &str) {
    println!("{} has the potato: {}!", player_name, potato);
}
