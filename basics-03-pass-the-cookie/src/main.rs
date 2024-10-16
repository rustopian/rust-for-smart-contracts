fn main() {
    let mut recipe = String::from("Grandma's secret recipe");
    println!("Fresh recipe: {}", recipe);

    // Alice wants to look at the recipe
    let alice = &recipe;
    // Bob wants to read the recipe
    let bob = &recipe;

    // Carol wants to modify the recipe
    // ...while Alice and Bob are looking at it!
    // NOTE: this line alone will compile, if the following println! lines
    // are removed, thanks to Non-Lexical Lifetimes (NLL).
    let carol = &mut recipe;
    carol.push_str(" with extra spices");

    // ...but if they do read it...
    println!("Alice looks at the recipe: {}", alice);
    println!("Bob reads the recipe: {}", bob);
    // We'll get:
    // cannot borrow `recipe` as mutable because it is also borrowed as immutable

    // Now that alice and bob won't read the recipe anymore,
    // we can do a mutable borrow...
    let dan = &mut recipe;

    dan.push_str(" and cranberry sauce");
    println!("Dan modifies the recipe.");

    // wait, can dan also change the recipe?
    let eva = &mut recipe;
    println!("Eva wants to modify the recipe from: {}", eva);

    // Yes, again due to Non-Lexical Lifetimes (NLL)!
    // Unless we...
    dan.push_str(" and pumpkin sauce");
    // This line will cause an error when dan tries to edit the recipe above,
    // since the compiler will see that dan is trying to borrow the recipe
    // while carol is still going to modify it.

    // Now that we aren't doing any more mutable borrow activity,
    // everybody can look at the recipe again. Before Non-Lexical Lifetimes,
    // `carol` `dan` and `eva` would have had to go out of scope first.
    // Alice wants to look at the recipe
    let alice = &recipe;
    // Bob wants to read the recipe
    let bob = &recipe;

    // Use alice and bob after the mutable borrow
    println!("Alice looks at the recipe: {}", alice);
    println!("Bob reads the recipe: {}", bob);

    // Display the final state of the recipe
    println!("Final recipe: {}", recipe);
}