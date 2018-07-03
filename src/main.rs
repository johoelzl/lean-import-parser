#[macro_use]
extern crate nom;

use std::{env, fs, str};

use nom::multispace;


// This is an ASCII only parser for Lean's import statements. Lean's --deps argument parser doesn't
// work in version 3.4.1.

#[inline]
fn is_name_char(c : char) -> bool {
  c.is_alphanumeric() || c == '_'
}

named!(whitespace( &str ) -> (), do_parse!(
  many0!(alt!(
      do_parse!(multispace >> ())
    | do_parse!(tag_s!("/-") >> take_until_and_consume!("-/") >> ())
    | do_parse!(tag_s!("--") >> take_until_either_and_consume!("\r\n") >> ()))) >>
  ()));

named!(word( &str ) -> Vec<&str>, do_parse!(
  whitespace >>
  n: separated_list!(tag_s!("."), take_while_s!(is_name_char)) >>
  (n)));

const PRELUDE : &'static str = "prelude";

const IMPORT : &'static str = "import";

const KEYWORDS : &'static [&'static str] = &[
  "abbreviation",
  "add_key_equivalence",
  "attribute",
  "axiom",
  "axioms",
  "class",
  "coinductive",
  "constant",
  "constants",
  "definition",
  "declare_trace",
  "example",
  "export",
  "hide",
  "include",
  "inductive",
  "infix",
  "infixl",
  "infixr",
  "init_quotient",
  "instance",
  "local",
  "meta",
  "mutual",
  "namespace",
  "noncomputable",
  "notation",
  "parameter",
  "parameters",
  "precedence",
  "prefix",
  "private",
  "protected",
  "postfix",
  "reserve",
  "run_cmd",
  "omit",
  "open",
  "section",
  "set_option",
  "structure",
  "theorem",
  "universe",
  "universes",
  "variable",
  "variables"
];

fn main() {
  let mut args = env::args();
  let _ = args.next().unwrap();
  let module = args.next().expect("filename expected as first argument");

  print!("file {:}\n", module);

  let mut contents : &str = &fs::read_to_string(module).expect("module not found");
  let mut prelude = false;
  let mut modules = Vec::new();

  loop {
    match word(&contents) {
      Ok((continuation, word)) => {
        contents = continuation;
        if word.len() == 1 {
          let identifier : &str = word.first().unwrap();
          if identifier == PRELUDE { prelude = true; }
          else if identifier == IMPORT { }
          else if KEYWORDS.contains(&identifier) { break; }
          else { modules.push(word); }
        } else {
          modules.push(word);
        }
      },
      Err(_e) => {
        panic!("parser error");
      }
    }
  }

  print!("Dependencies (prelude: {:})\n", prelude);
  for name in modules {
    let n : String = name.join(".");
    print!("  {:}\n", n);
  }
}
