#[macro_use]
extern crate nom;

use nom::multispace;

/* horrible ASCII parser for UTF-8 input... */

named!(multi_line_comment( &[u8] ) -> (), do_parse!(tag!("/-") >> take_until!("-/") >> ()));

named!(line_comment( &[u8] ) -> (), do_parse!(tag!("--") >> take_until!("\r") >> ()));

named!(whitespace( &[u8] ) -> (),
  do_parse!(many0!(alt!(do_parse!(multispace >> ()) | multi_line_comment | line_comment)) >> ()));

named!(prelude( &[u8] ) -> (), do_parse!(tag!("prelude") >> ()));

named!(import( &[u8] ) -> (), do_parse!(tag!("import") >> ()));

const KEYWORDS : &'static [&'static str] = &[
  "conjecture",
  "constant",
  "constants",
  "corollary",
  "declare_trace",
  "def",
  "hypothesis",
  "lemma",
  "meta",
  "noncomputable",
  "parameter",
  "parameters",
  "protected",
  "private",
  "variable",
  "variables",
  "theorem",
  "example",
  "open",
  "export",
  "axiom",
  "axioms",
  "inductive",
  "structure",
  "universe",
  "universes",
  "precedence",
  "reserve",
  "infix",
  "infixl",
  "infixr",
  "notation",
  "postfix",
  "prefix",
  "instance",
  "namespace",
  "section",
  "attribute",
  "local",
  "set_option",
  "include",
  "omit",
  "class",
  "mutual",
  "run_command"
];


fn main() {
    println!("Hello, world!");
}
