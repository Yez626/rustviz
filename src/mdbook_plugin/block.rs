//! Parser for Rustviz code blocks within Markdown.

use std::{hash::Hash, ops::Range};

use nom::{
  branch::alt,
  bytes::complete::{tag, take_until},
  character::complete::{anychar, char, none_of},
  combinator::map,
  multi::{many0, separated_list1},
  sequence::{preceded, separated_pair, tuple},
  IResult,
};

use nom_locate::LocatedSpan;

#[derive(PartialEq, Hash, Debug, Clone)]
pub struct RustvizBlock {
  pub operations: Vec<String>,
  pub config: Vec<(String, String)>,
  pub code: String,
  pub annotations: RustvizAnnotations,
}