use std::collections::{HashMap};
use std::iter::Iterator;

#[derive (PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Id { id: usize }

#[derive (PartialEq)]
pub struct Module {
  id: Id,
  prelude: bool,
  dependencies: Vec<Id>
}

impl Module {
  pub fn prelude(&self) -> bool { self.prelude }
  pub fn id(&self) -> Id { self.id }
  pub fn dependencies(&self) -> &Vec<Id> { &self.dependencies }
}

pub struct Graph {
  names: Vec<String>,
  id: HashMap<String, Id>,
  modules: HashMap<Id, Module>
}

impl Graph {
  pub fn new() -> Graph {
    Graph { names: Vec::new(), id: HashMap::new(), modules: HashMap::new() }
  }

  pub fn register_name(&mut self, name: &str) -> Id {
    self.id.get(name).cloned().unwrap_or_else(|| {
      let id = Id { id: self.names.len() };
      self.names.push(name.to_string());
      self.id.insert(name.to_string(), id);
      id
    })
  }

  pub fn register_module(&mut self, id: Id, prelude: bool, dependencies: Vec<Id>) {
    if self.modules.insert(id, Module { id, prelude, dependencies }).is_some() {
      panic!("Module for id {} ({}) registered twice.", id.id, self.get_name(id));
    }
  }

  pub fn get_name(&self, id: Id) -> &String {
    self.names.get(id.id).unwrap()
  }

  pub fn get_module (&self, id: Id) -> Option<&Module> {
    self.modules.get(&id)
  }

  pub fn modules (&self) -> impl Iterator<Item = &Module> {
    self.modules.values()
  }

  pub fn iter_edges<'a> (&'a self) -> impl Iterator<Item = (&'a Module, Id)> + 'a {
    self.modules.values().flat_map(|m| m.dependencies.iter().map(move |&d| (m, d)))
  }

}
