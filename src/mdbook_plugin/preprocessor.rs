//! Parser for Rustviz code blocks within Markdown.

use std::{
    collections::HashMap,
    fmt::Write,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    sync::RwLock,
    time::Duration,
  };

use anyhow::{bail, Result};
use rayon::prelude::*;
use tempfile::tempdir;
use wait_timeout::ChildExt;


pub struct RustvizPreprocessor {
    miri_sysroot: PathBuf,
    target_libdir: PathBuf,
    cache: RwLock<Cache<RustvizBlock, String>>,
  }


/// Runs cargo-aquascope on code from a given Rustviz block.
fn run_aquascope(&self, block: &RustvizBlock) -> Result<String>
{
    
}

