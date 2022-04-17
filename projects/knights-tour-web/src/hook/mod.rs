mod display;
mod iter;

use dioxus::core::ScopeState;
use knights_tour::{ChessRole, Chessboard};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

pub fn use_chessboard(cx: &ScopeState) -> &mut UseChessboard {
    let hook = UseChessboard::new(cx);
    cx.use_hook(|| hook)
}

/// Local storage effect handler
pub struct UseChessboard {
    data: Rc<RefCell<Chessboard>>,
    // listen_storage: Option<EventListener>,
}

impl UseChessboard {
    fn new(_: &ScopeState) -> Self {
        UseChessboard {
            data: Rc::new(RefCell::new(Chessboard {
                role: ChessRole::Knight,
                size: (0, 0),
                start: (0, 0),
                back_to_start: false,
            })),
        }
    }
}

impl UseChessboard {
    pub fn get_w(&self) -> usize {
        self.data.borrow().size.0
    }
    pub fn set_w(&self, value: usize) {
        self.data.borrow_mut().size.0 = value
    }
    pub fn get_h(&self) -> usize {
        self.data.borrow().size.1
    }
    pub fn set_h(&self, value: usize) {
        self.data.borrow_mut().size.1 = value
    }
    pub fn get_x(&self) -> usize {
        self.data.borrow().start.0
    }
    pub fn set_x(&self, value: usize) {
        self.data.borrow_mut().start.0 = value
    }
    pub fn get_y(&self) -> usize {
        self.data.borrow().start.1
    }
    pub fn set_y(&self, value: usize) {
        self.data.borrow_mut().start.1 = value
    }
    pub fn get_back_to_start(&self) -> bool {
        self.data.borrow().back_to_start
    }
    pub fn set_back_to_start(&self, value: bool) {
        self.data.borrow_mut().back_to_start = value
    }
}
