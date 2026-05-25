use ghost_cells::ghost_cell::{GhostCell, GhostToken};
use std::mem;

fn main() {
    GhostToken::new(|mut token| {
        let cell_a = GhostCell::new("Apple");
        let cell_b = GhostCell::new("Banana");

        println!(
            "Before: A={}, B={}",
            cell_a.borrow(&token),
            cell_b.borrow(&token)
        );

        let (val_a, val_b) = token.borrow_mut2(&cell_a, &cell_b);
        mem::swap(val_a, val_b);

        println!(
            "After: A={}, B={}",
            cell_a.borrow(&token),
            cell_b.borrow(&token)
        );
    });
}
