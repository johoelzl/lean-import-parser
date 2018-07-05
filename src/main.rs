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
      let deps = dependencies.iter().map(|v| graph.register_name(&v.join("."))).collect();
      graph.register_module(id, prelude, deps);
    }
  }
}

const CHECK_PRELUDE: bool = false;
const CHECK_LOCAL_IMPORT: bool = true;
const PRINT_GRAPH: bool = true;

fn check_find_all(graph : &Graph) {
  println!("check if all modules are found");
  for (m, d) in graph.iter_edges() {
    if graph.get_module(d).is_none() {
      println!("module {} (in {}) not found", graph.get_name(d), graph.get_name(m.id()));
    }
  }
}

fn check_prelude(graph : &Graph) {
    println!("which non-prelue imports a prelude");
    for (m, id) in graph.iter_edges() {
      let d = graph.get_module(id).unwrap();
      if !m.prelude() && d.prelude() {
            println!("module {} imports {}", graph.get_name(m.id()), graph.get_name(d.id()));
      }
    }
}

fn check_local_import(graph : &Graph) {
  println!("check if a module uses local \".\" import syntax");
  for (m, id) in graph.iter_edges() {
    if graph.get_name(id).starts_with(".") {
      println!("local module syntax used in {} for {}", graph.get_name(m.id()), graph.get_name(id));
      panic!("TODO: local module syntax not supported");
    }
  }
}

fn compute_transitive_map(
  graph: &Graph,
  m: &Module,
  history: &Vec<Id>,
  transitive_map: &mut HashMap<Id, Vec<Id>>)
{
  if transitive_map.contains_key(&m.id()) { return; }

  if history.iter().any(|i| *i == m.id()) {
    panic!("cycle detected");
  }

  let mut new_history = history.clone();
  new_history.push(m.id());

  let mut transitive : Vec<Id> = Vec::new();
  for d in m.dependencies().iter() {
    compute_transitive_map(graph, graph.get_module(*d).unwrap(), &new_history, transitive_map);
    transitive.extend(transitive_map.get(d).unwrap());
  }

  transitive_map.insert(m.id(), transitive);
}

fn print_graph(graph : &Graph) {
  println!("print graph");

  let mut transitive_map: HashMap<Id, Vec<Id>> = HashMap::new();
  let mut direct_dependencies: HashMap<Id, Vec<Id>> = HashMap::new();

  for module in graph.modules() {
    compute_transitive_map(graph, module, &Vec::new(), &mut transitive_map);
  }

  for module in graph.modules() {
    let old : HashSet<Id> = module.dependencies().iter().cloned().collect();
    let all_children : HashSet<Id> =
      module
        .dependencies()
        .into_iter()
        .flat_map(|d| transitive_map.get(&d).unwrap().into_iter())
        .cloned()
        .collect();
    let new : Vec<Id> = old.difference(&all_children).cloned().collect();
    direct_dependencies.insert(module.id(), new);
  }

}

fn main() {
  let mut args = env::args();
  let _program_name = args.next().unwrap();
  let mut graph = Graph::new();
  for dir in args { scan_path(&mut graph, &Vec::new(), Path::new(&dir)); }

  check_find_all(&graph);
  if CHECK_PRELUDE { check_prelude(&graph); }
  if CHECK_LOCAL_IMPORT { check_local_import(&graph); }
  if PRINT_GRAPH { print_graph(&graph); }
}
