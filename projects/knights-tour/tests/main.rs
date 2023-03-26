use knights_tour::{ChessRole, Chessboard, SvgRender};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_knights_tour() {
    let (m, n) = (8, 8);
    let knights = Chessboard::new(m, n);
    std::fs::create_dir_all("target/").unwrap();
    for (index, state) in knights.initial_state().warnsdorff_rule().take(10).enumerate() {
        let render = SvgRender::default();
        let svg = state.draw_svg(&render);
        let file = format!("target/knights{m}x{n}_{}.svg", index);
        std::fs::write(file, svg).unwrap();
    }
}

#[test]
fn test_pawns_tour() {
    let knights = Chessboard::new(10, 10).with_role(ChessRole::Pawn).walk(true);
    std::fs::create_dir_all("target/").unwrap();
    for (index, state) in knights.into_iter().enumerate() {
        let render = SvgRender::default();
        let svg = state.draw_svg(&render);
        let file = format!("target/pawn10x10_{}.svg", index);
        std::fs::write(file, svg).unwrap();
    }
}
