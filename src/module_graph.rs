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
  pub fn prelude(&self) : bool { self.prelude }
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
    match self.id.get(name) {
      None => {
        let id = Id { id: self.names.len() };
        self.names.push(name.to_string());
        self.id.insert(name.to_string(), id);
        id
      },
      Some(id) => *id
    }
  }

  pub fn register_module(&mut self, module: Module) {
    if self.modules.insert(module.id, module).is_none() {
      panic!("Module for id {} registered twice.", module.id.id);
    }
  }

  pub fn get_name(&self, id: Id) -> String {
    self.names.get(id.id).unwrap().clone()
  }

  pub fn get_module<'a> (&'a self, id: Id) -> &'a Module {
    self.modules.get(&id).unwrap()
  }

  pub fn find_module(&self, name: &str) -> Option<Id> {
    self.id.get(&name.to_string()).map (|x| *x)
  }

  pub fn iter_edges<'a>(&'a self) -> impl Iterator<Item = (Id, Id)> + 'a {
    self.modules.values().flat_map(|m| m.dependencies.iter().map(|d| (m.id, *d)))
  }

}
