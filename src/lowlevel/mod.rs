// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Low Level RDF Interface

pub mod memory;

/// An RDF Term
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum Term<'t> {
    NamedNode {
        value: &'t str,
    },
    BlankNode {
        value: &'t str,
    },
    Literal {
        value: &'t str,
        language: Option<&'t str>,
        datatype: Option<&'t str>,
    },
}

/// An RDF Statement
///
/// A statement differs from a [`Triple`] in that it contains
/// an additional member to refer to the graph to which it
/// belongs.
///
/// Lifetime parameter `'t` controls the lifetime of the text
/// data stored within the terms. This allows a reader to not
/// be required to copy data into separate string objects
/// while reading and parsing RDF data files.
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub struct Statement<'t> {
    pub subject: Term<'t>,
    pub predicate: Term<'t>,
    pub object: Term<'t>,
    pub graph: Option<Term<'t>>,
}

/// An RDF Triple
///
/// Lifetime parameter `'t` controls the lifetime of the text
/// data stored within the terms. This allows a reader to not
/// be required to copy data into separate string objects
/// while reading and parsing RDF data files.
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub struct Triple<'t> {
    pub subject: Term<'t>,
    pub predicate: Term<'t>,
    pub object: Term<'t>,
}

/// A trait that can be implemented to provide a way to run
/// an action whenever a triple is added to a graph.
pub trait TripleAction {
    /// The method to run when a triple is added to a graph.
    fn run(&self, triple: &Triple, graph: &Graph);
}

/// A collection of triples.
pub trait Graph<'t> {
    /// Add an action to be run on each triple to the graph.
    ///
    /// If `run_on_existing` is `true`, then the action will be run on
    /// all triples currently contained in the graph.
    fn add_action(&mut self, action: Box<TripleAction>, run_on_existing: bool);

    /// Add a triple to the graph.
    ///
    /// If actions have been added to the graph, then they will be run
    /// before the triple is pushed into the graph's storage.
    fn add(&mut self, triple: Triple<'t>);

    /// Add a statement to the graph.
    ///
    /// If actions have been added to the graph, then they will be run
    /// before the statement is pushed into the graph's storage.
    ///
    /// A statement should only be added to the graph to which it belongs.
    fn add_statement(&mut self, statement: Statement<'t>) {
        self.add(Triple {
            subject: statement.subject,
            predicate: statement.predicate,
            object: statement.object,
        });
    }
}
