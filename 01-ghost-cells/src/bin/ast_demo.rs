use ghost_cells::ghost_cell::{GhostCell, GhostToken};
use typed_arena::Arena;

struct Node<'a, 'id> {
    name: String,
    children: Vec<&'a GhostCell<'id, Node<'a, 'id>>>,
}

fn main() {
    GhostToken::new(|mut token| {
        let arena = Arena::new();

        // Create nodes
        let root = arena.alloc(GhostCell::new(Node {
            name: "Root".to_string(),
            children: vec![],
        }));
        let child1 = arena.alloc(GhostCell::new(Node {
            name: "Child 1".to_string(),
            children: vec![],
        }));

        // Safely mutate the graph to create links
        root.borrow_mut(&mut token).children.push(child1);

        // Prove we can read it back out
        let root_ref = root.borrow(&token);
        println!("Root is: {}", root_ref.name);
        println!("Child is: {}", root_ref.children[0].borrow(&token).name);
    });
}
