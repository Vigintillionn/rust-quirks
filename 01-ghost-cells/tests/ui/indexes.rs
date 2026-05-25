use ghost_cells::branded_arena::Arena;

fn main() {
    Arena::new(|mut arena_a| {
        let index_a = arena_a.insert("Node A");

        Arena::new(|mut arena_b| {
            arena_b.insert("Node B");
            let _node = arena_b.get(index_a);
        });
    });
}
