use super::*;
use std::fmt::Formatter;

impl Display for KnightsTourState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("KnightsTourState")
            .field(&self.size_x)
            .field(&self.size_y)
            .field(&self.current_x)
            .field(&self.current_y)
            .field(&self.back_to_start)
            .field(&self.path)
            .finish()
    }
}
