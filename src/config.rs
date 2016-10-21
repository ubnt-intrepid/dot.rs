use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use toml;
use shellexpand;
use regex;


#[allow(dead_code)]
pub struct Config {
  repo: String,
  dotdir: String,
  linkfiles: BTreeMap<String, Vec<(String, String)>>,
}

impl Config {
  pub fn new<P: AsRef<Path>>(path: P) -> Config {
    let mut buf = Vec::new();
    File::open(path).unwrap().read_to_end(&mut buf).unwrap();
    let content = String::from_utf8_lossy(&buf[..]).into_owned();

    let parsed: toml::Table = toml::Parser::new(&content).parse().unwrap();

    let repo = parsed.get("clone_repository").unwrap().as_str().unwrap().to_owned();
    let dotdir = parsed.get("dotdir").unwrap().as_str().unwrap().to_owned();

    let mut buf = BTreeMap::new();
    for linkfile in parsed.get("linkfiles")
      .unwrap()
      .as_slice()
      .unwrap()
      .iter()
      .map(|v| shellexpand::full(v.as_str().unwrap()).unwrap().into_owned()) {
      buf.insert(linkfile.clone(), parse_linkfile(&linkfile));
    }

    Config {
      repo: repo,
      dotdir: dotdir,
      linkfiles: buf,
    }
  }

  pub fn get_linkfiles<'a>(&'a self) -> &'a BTreeMap<String, Vec<(String, String)>> {
    &self.linkfiles
  }
}

fn parse_linkfile<P: AsRef<Path>>(linkfile: P) -> Vec<(String, String)> {
  let file = File::open(linkfile.as_ref()).unwrap();
  let file = BufReader::new(file);

  let re = regex::Regex::new(r"^\s*#.*$|^\s*$").unwrap();

  let mut buf = Vec::new();

  for line in file.lines() {
    let line = re.replace(&line.unwrap(), "");
    if line == "" {
      continue;
    }

    let token: Vec<_> = line.split(",").map(|s| s.trim().to_owned()).collect();
    buf.push((token[0].clone(), token[1].clone()));
  }

  buf
}
