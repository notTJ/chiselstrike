// SPDX-FileCopyrightText: © 2021 ChiselStrike <info@chiselstrike.com>

use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound;
use std::path::{Path, PathBuf};
use yaml_rust::YamlLoader;

#[derive(Clone)]
pub(crate) struct Policy {
    /// How this policy transforms values read from storage.
    pub(crate) transform: fn(Value) -> Value,

    /// This policy doesn't apply when the request URI matches.
    pub(crate) except_uri: regex::Regex,
}

/// Maps labels to their applicable policies.
pub(crate) type LabelPolicies = HashMap<String, Policy>;

/// Maps a field name to the transformation we apply to that field's values.
pub(crate) type FieldPolicies = HashMap<String, fn(Value) -> Value>;

#[derive(Clone, Default)]
pub(crate) struct UserAuthorization {
    /// A user is authorized to access a path if the username matches the regex for the longest path prefix present
    /// here.
    paths: BTreeMap<PathBuf, regex::Regex>,
}

impl UserAuthorization {
    /// Is this username allowed to execute the endpoint at this path?
    pub fn is_allowed(&self, username: Option<String>, path: &Path) -> bool {
        let path_range = (Bound::Unbounded, Bound::Included(path));
        let map_range = self.paths.range::<Path, _>(path_range);
        for (p, u) in map_range.rev() {
            if path.starts_with(p) {
                return match username {
                    None => false, // Must be logged in if path specified a regex.
                    Some(username) => u.is_match(&username),
                };
            }
        }
        true
    }

    /// Authorizes users matching a regex to execute any endpoint under this path.  Longer paths override existing
    /// prefixes.  Error if this same path has already been added.
    pub fn add(&mut self, path: &str, users: regex::Regex) -> Result<(), anyhow::Error> {
        if self.paths.insert(path.into(), users).is_some() {
            anyhow::bail!("Repeated path in user authorization: {:?}", path);
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub(crate) struct Policies {
    pub(crate) labels: LabelPolicies,
    pub(crate) user_authorization: UserAuthorization,
}

impl Policies {
    pub(crate) fn from_yaml<S: AsRef<str>>(config: S) -> anyhow::Result<Self> {
        let mut policies = Self::default();
        let mut labels = vec![];

        let docs = YamlLoader::load_from_str(config.as_ref())?;
        for config in docs.iter() {
            for label in config["labels"].as_vec().get_or_insert(&[].into()).iter() {
                let name = label["name"].as_str().ok_or_else(|| {
                    anyhow::anyhow!("couldn't parse yaml: label without a name: {:?}", label)
                })?;

                labels.push(name.to_owned());
                debug!("Applying policy for label {:?}", name);

                match label["transform"].as_str() {
                    Some("anonymize") => {
                        let pattern = label["except_uri"].as_str().unwrap_or("^$"); // ^$ never matches; each path has at least a '/' in it.
                        policies.labels.insert(
                            name.to_owned(),
                            Policy {
                                transform: crate::policies::anonymize,
                                except_uri: regex::Regex::new(pattern)?,
                            },
                        );
                    }
                    Some(x) => {
                        anyhow::bail!("unknown transform: {} for label {}", x, name);
                    }
                    None => {}
                };
            }
            for endpoint in config["endpoints"]
                .as_vec()
                .get_or_insert(&[].into())
                .iter()
            {
                if let Some(path) = endpoint["path"].as_str() {
                    if let Some(users) = endpoint["users"].as_str() {
                        policies
                            .user_authorization
                            .add(path, regex::Regex::new(users)?)?;
                    }
                }
            }
        }
        Ok(policies)
    }
}

pub(crate) fn anonymize(_: Value) -> Value {
    // TODO: use type-specific anonymization.
    json!("xxxxx")
}
