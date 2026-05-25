use ghost_cells::ghost_cell::{GhostCell, GhostToken};

fn main() {
    GhostToken::new(|mut token| {
        let cell_a = GhostCell::new(10);
        let cell_b = GhostCell::new(20);

        // We cannot borrow `token` mutably twice.
        let mut_a = cell_a.borrow_mut(&mut token);
        let mut_b = cell_b.borrow_mut(&mut token);

        *mut_a += 1;
        *mut_b += 1;
    });
}
