#[macro_use]
extern crate nom;

mod parser;
mod module_graph;

use std::path::Path;
use std::env;
use std::fs;
use std::ffi;
use std::collections::{HashMap, HashSet};

use module_graph::{Id, Module, Graph};

fn scan_path(graph : &mut module_graph::Graph, name : &Vec<String>, p : &Path) {
  for entry in fs::read_dir(p).unwrap() {
    let e = entry.unwrap();
    let m = e.metadata().unwrap();
    let p = e.path();
    let mut n = name.clone();
    let stem = e.path().file_stem().unwrap().to_string_lossy().to_string();
    if stem != "default" { n.push(stem) };
    if m.is_dir() {
      scan_path(graph, &n, &e.path());
    } else if p.extension() == Some(ffi::OsStr::new("lean")) {
      let (prelude, dependencies) = parser::parse(&p.to_string_lossy());
      let id = graph.register_name(&n.join("."));
      graph.register_module(Module {
        id, prelude,
        dependencies: dependencies.iter().map(|v| graph.register_name(&v.join("."))).collect()
      });
    }
  }
}

const CHECK_FIND_ALL: bool = false;
const CHECK_PRELUDE: bool = false;
const CHECK_LOCAL_IMPORT: bool = true;
const PRINT_GRAPH: bool = true;

fn check_prelude(graph : &Graph) {
    println!("which non-prelue imports a prelude");
    for (m_id, d_id) in graph.iter_edges() {
      let m = graph.get_module(m_id);
      let d = graph.get_module(d_id);
      if !m.prelude() && d.prelude() {
            println!("module {} imports {}", m.name, d.name);
      }
    }
}

fn check_find_all(modules : &Graph) {
  println!("check if all modules are found");
  for (_n, m) in modules.iter() {
    for name in m.dependencies.iter() {
      if ! modules.contains_key(name) {
        println!("module {} (in {}) not found", name.join("."), m.name.join("."));
      }
    }
  }
}

fn check_local_import(modules : &Graph) {
  println!("check if a module uses local \".\" import syntax");
  for (n, m) in modules.iter() {
    for name in m.dependencies.iter() {
      if name.first().unwrap() == "" {
        println!("local module syntax used in {} for {}", n.join("."), name.join("."));
        panic!("TODO: local module syntax not supported");
      }
    }
  }
}

fn compute_full_dependencies(
  name: &Name,
  modules: &Graph,
  history: &Vec<Name>,
  full_dependencies: &mut HashMap<Name, Vec<Name>>) {
  if full_dependencies.contains_key(name) { return; }
  { let cycle : Vec<Name> = history.iter().skip_while(|&n| n != name).cloned().collect();
    if cycle.len() > 0 {
      panic!("cycle detected");
    }
  }

  let mut full : Vec<Name> = Vec::new();
  let m = modules.get(name).unwrap();
  let mut new_history = history.clone();
  new_history.push(name.clone());
  for dep in m.dependencies.iter() {
    compute_full_dependencies(dep, modules, &new_history, full_dependencies);
    full.append(&mut full_dependencies.get(dep).unwrap().clone());
  }
  full_dependencies.insert(name.clone(), full);
}

fn print_graph(modules : &Graph) {
  println!("print graph");

  let mut full_dependencies: HashMap<Name, Vec<Name>> = HashMap::new();
  let mut direct_dependencies: HashMap<Name, Vec<Name>> = HashMap::new();

  for (name, _module) in modules.iter() {
    compute_full_dependencies(&name, modules, &Vec::new(), &mut full_dependencies);
  }

  for (name, module) in modules.iter() {
    let old : HashSet<Name> = module.dependencies.iter().cloned().collect();
    let all_children : HashSet<Name> =
      module.dependencies.iter().flat_map(|d| full_dependencies.get(d).unwrap().iter()).cloned().collect();
    let new : Vec<Name> = old.difference(&all_children).map(|n| n.to_vec()).collect();
    direct_dependencies.insert(name.clone(), new);
  }



}

fn main() {
  let mut args = env::args();
  let _program_name = args.next().unwrap();
  let mut graph = Graph::new();
  for dir in args { scan_path(&mut graph, &Vec::new(), Path::new(&dir)); }

  if CHECK_FIND_ALL { check_find_all(&graph); }
  if CHECK_PRELUDE { check_prelude(&graph); }
  if CHECK_LOCAL_IMPORT { check_local_import(&graph); }
  if PRINT_GRAPH { print_graph(&graph); }
}
