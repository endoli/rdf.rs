// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Memory-backed Low-Level Graph
//!
//! ...

use lowlevel::*;

/// Memory-backed `Graph` implementation.
#[derive(Default)]
pub struct MemoryGraph<'t> {
    triples: Vec<Triple<'t>>,
    actions: Vec<Box<TripleAction>>,
}

impl<'t> MemoryGraph<'t> {
    /// Create a new, empty graph.
    pub fn new() -> Self {
        MemoryGraph::default()
    }
}

impl<'t> Graph<'t> for MemoryGraph<'t> {
    /// Add an action to be run on each triple to the graph.
    ///
    /// If `run_on_existing` is `true`, then the action will be run on
    /// all triples currently contained in the graph.
    fn add_action(&mut self, action: Box<TripleAction>, run_on_existing: bool) {
        if run_on_existing {
            for triple in &self.triples {
                action.run(triple, self);
            }
        }
        self.actions.push(action);
    }

    /// Add a triple to the graph.
    ///
    /// If actions have been added to the graph, then they will be run
    /// before the triple is pushed into the graph's storage.
    fn add(&mut self, triple: Triple<'t>) {
        for action in &self.actions {
            action.run(&triple, self);
        }
        self.triples.push(triple);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn create_graph() {
        let g = MemoryGraph::new();
        assert!(g.triples.is_empty());
        assert!(g.actions.is_empty());
    }

    #[test]
    fn add_triple() {
        let mut g = MemoryGraph::new();
        g.add(Triple {
            subject: Term::NamedNode { value: "a" },
            predicate: Term::NamedNode { value: "b" },
            object: Term::NamedNode { value: "c" },
        });
        assert_eq!(g.triples.len(), 1);
    }

    struct CountingTripleAction {
        pub count: Rc<Cell<i32>>,
    }

    impl TripleAction for CountingTripleAction {
        fn run(&self, _triple: &Triple, _graph: &Graph) {
            self.count.set(self.count.get() + 1);
        }
    }

    #[test]
    fn run_action_on_add_triple() {
        let count = Rc::new(Cell::<i32>::new(0));
        let mut g = MemoryGraph::new();
        let a = Box::new(CountingTripleAction {
            count: count.clone(),
        });
        g.add_action(a, false);
        g.add(Triple {
            subject: Term::NamedNode { value: "a" },
            predicate: Term::NamedNode { value: "b" },
            object: Term::NamedNode { value: "c" },
        });
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn run_action_on_add_action() {
        let count = Rc::new(Cell::<i32>::new(0));
        let mut g = MemoryGraph::new();
        g.add(Triple {
            subject: Term::NamedNode { value: "a" },
            predicate: Term::NamedNode { value: "b" },
            object: Term::NamedNode { value: "c" },
        });
        let a = Box::new(CountingTripleAction {
            count: count.clone(),
        });
        g.add_action(a, true);
        assert_eq!(count.get(), 1);
        g.add(Triple {
            subject: Term::NamedNode { value: "d" },
            predicate: Term::NamedNode { value: "e" },
            object: Term::NamedNode { value: "f" },
        });
        assert_eq!(count.get(), 2);
    }
}
