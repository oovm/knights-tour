use knights_tour::KnightsTour;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_knights_tour() {
    let state = KnightsTour::new(5, 5).with_back_to_start(true);
    for i in state.into_iter().take(2) {
        println!("{}", i);
        println!("{:#?}", i);
    }
}
