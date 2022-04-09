use knights_tour::{KnightsTour, SvgRender};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_knights_tour() {
    let state = KnightsTour::new(5, 5).with_back_to_start(false);
    for i in state.into_iter().take(2) {
        // write string to file\
        let render = SvgRender::default();
        let svg = i.draw_svg(&render);
        let file = format!("s.svg");
        std::fs::write(file, svg).unwrap();
        println!("{}", i);
        println!("{:#?}", i);
    }
}
