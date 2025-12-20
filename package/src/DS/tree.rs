use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct UndoTreeNode {
    state: String,
    parent: Option<Rc<RefCell<UndoTreeNode>>>,
    children: Vec<Rc<RefCell<UndoTreeNode>>>,
}

#[derive(Debug, Clone)]
pub struct UndoTree {
    current: Rc<RefCell<UndoTreeNode>>,
}

impl UndoTree {
    pub fn new(initial_state: &str) -> Self {
        let root = Rc::new(RefCell::new(UndoTreeNode {
            state: initial_state.to_string(),
            parent: None,
            children: vec![],
        }));

        UndoTree { current: root }
    }

    /// Saves a new state into the history tree
    pub fn commit(&mut self, new_state: String) {
        let new_node = Rc::new(RefCell::new(UndoTreeNode {
            state: new_state,
            parent: Some(Rc::clone(&self.current)),
            children: vec![],
        }));

        // Add this new state as a child of where we are currently
        self.current
            .borrow_mut()
            .children
            .push(Rc::clone(&new_node));
        // Move our pointer to the new state
        self.current = new_node;
    }

    /// Moves back one step in history
    pub fn undo(&mut self) {
        let parent = self.current.borrow().parent.as_ref().map(Rc::clone);
        if let Some(p) = parent {
            self.current = p;
        } else {
            println!("Root reached. Cannot undo further.");
        }
    }

    /// Moves forward to the most recent branch created
    pub fn redo(&mut self) {
        // Automatically picks the LAST child (the most recent "future" created)
        let child = self.current.borrow().children.last().map(Rc::clone);

        if let Some(c) = child {
            self.current = c;
        } else {
            println!("No redo history available from this point.");
        }
    }

    pub fn current_state(&self) -> String {
        self.current.borrow().state.clone()
    }
}
