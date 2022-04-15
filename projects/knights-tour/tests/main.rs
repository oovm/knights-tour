use knights_tour::{ChessRole, Chessboard, SvgRender};
use std::fs::create_dir_all;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_knights_tour() {
    let knights = Chessboard::new(8, 8).with_role(ChessRole::Pawn).walk(false);
    create_dir_all("target/").unwrap();
    for (index, state) in knights.into_iter().take(10).enumerate() {
        // write string to file\
        let render = SvgRender::default();
        let svg = state.draw_svg(&render);
        let file = format!("target/step_{}.svg", index);
        std::fs::write(file, svg).unwrap();
    }
}
