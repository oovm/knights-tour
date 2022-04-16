use knights_tour::{ChessRole, Chessboard, SvgRender};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_knights_tour() {
    let knights = Chessboard::new(8, 8);
    std::fs::create_dir_all("target/").unwrap();
    for (index, state) in knights.into_iter().take(10).enumerate() {
        // write string to file\
        let render = SvgRender::default();
        let svg = state.draw_svg(&render);
        let file = format!("target/knights8x8_{}.svg", index);
        std::fs::write(file, svg).unwrap();
    }
}

#[test]
fn test_pawns_tour() {
    let knights = Chessboard::new(8, 8).with_role(ChessRole::Pawn).walk(false);
    std::fs::create_dir_all("target/").unwrap();
    for (index, state) in knights.into_iter().take(10).enumerate() {
        let render = SvgRender::default();
        let svg = state.draw_svg(&render);
        let file = format!("target/pawn8x8_{}.svg", index);
        std::fs::write(file, svg).unwrap();
    }
}
