use anyhow::{Context, Result};
use glob::glob;
use ioprio::{BePriorityLevel, Class, RtPriorityLevel};
use regex::Regex;
use std::{collections::HashMap, fs, hash::Hash};

use merge::Merge;
use serde::Deserialize;

#[derive(Deserialize, Merge)]
#[serde(default)]
pub(crate) struct Config {
    #[merge(strategy = hash_append)]
    #[serde(rename = "group")]
    pub(crate) groups: HashMap<String, Group>,

    #[merge(strategy = merge::vec::append)]
    #[serde(rename = "rule")]
    pub(crate) rules: Vec<Rule>,

    #[merge(skip)]
    pub(crate) poll_interval: u64,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) nice: i64,
    pub(crate) io_class: IoClass,
}

#[derive(Deserialize, Default, Clone)]
pub enum IoClass {
    Realtime(u8),
    BestEffort(u8),
    Idle,

    #[default]
    None,
}

#[derive(Deserialize)]
pub(crate) struct Rule {
    #[serde(with = "serde_regex")]
    pub(crate) process: Regex,

    pub(crate) group: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            groups: HashMap::new(),
            rules: vec![],
            poll_interval: 5,
        }
    }
}

impl Config {
    pub(crate) fn read_file(path: &str) -> Result<Self> {
        Ok(toml::from_slice(
            fs::read(path)
                .context("Failed to read the main config file")?
                .as_slice(),
        )
        .context("Failed to parse config file")?)
    }

    pub(crate) fn merge_from_directory(&mut self, directory: &str) -> anyhow::Result<()> {
        let entries =
            glob(&format!("{directory}/**/*.toml")).context("Failed to parse glob pattern")?;

        for entry in entries.filter_map(|p| p.ok()) {
            log::debug!("Trying to parse {entry:?}");

            let content = fs::read(&entry).context(format!("Failed to read {entry:?}"))?;
            let conf: Self = toml::from_slice(content.as_slice())
                .context(format!("Failed to parse {entry:?}"))?;

            log::debug!("Loaded config {entry:?}");
            self.merge(conf);
        }
        Ok(())
    }
}

fn hash_append<A: Hash + Eq, B>(left: &mut HashMap<A, B>, right: HashMap<A, B>) {
    for (key, value) in right {
        left.insert(key, value);
    }
}

impl TryFrom<IoClass> for Class {
    type Error = ();

    fn try_from(value: IoClass) -> Result<Self, Self::Error> {
        use IoClass::{BestEffort, Idle, None, Realtime};

        Ok(match value {
            None | Idle => Class::Idle,
            BestEffort(data) => {
                Class::BestEffort(BePriorityLevel::from_level(data).ok_or_else(|| ())?)
            }
            Realtime(data) => Class::Realtime(RtPriorityLevel::from_level(data).ok_or_else(|| ())?),
        })
    }
}
