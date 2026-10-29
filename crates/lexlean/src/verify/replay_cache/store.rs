//! The replay cache store (SPEC.md §22.13): one canonical JSON entry per
//! recorded unit, found by the unit's key.

use std::io::Read;

use camino::{Utf8Path, Utf8PathBuf};

use crate::artifact::canonical_json::Json;
use crate::artifact::digest_set::{DigestSet, DigestSetWriter};
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::verify::replay_cache::key::{Granularity, ReplayEnvironment};

const ENTRY_SPEC: &str = "lexlean/replay-cache-entry/1";

/// The executable that produced a result, named by the DigestSet of its bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutableIdentity {
    /// The tool's name, `leanchecker` for a replay.
    pub tool: String,
    /// Both digests of the executable's bytes.
    pub digest: DigestSet,
}

/// Hash an executable's bytes once, both ways.
pub fn executable_identity(tool: &str, path: &Utf8Path) -> Result<ExecutableIdentity, Diagnostic> {
    let io_error = |error: std::io::Error| {
        Diagnostic::new(
            code!("LLV7010"),
            format!("cannot read the executable `{path}`: {error}"),
        )
    };
    let mut file = std::fs::File::open(path.as_std_path()).map_err(io_error)?;
    let mut writer = DigestSetWriter::new();
    let mut buffer = vec![0u8; 1 << 16];
    loop {
        let read = file.read(&mut buffer).map_err(io_error)?;
        if read == 0 {
            break;
        }
        writer.update(&buffer[..read]);
    }
    Ok(ExecutableIdentity {
        tool: tool.to_owned(),
        digest: writer.finish(),
    })
}

/// What a successful replay left behind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayResult {
    /// Always zero in a recorded entry: a failed replay is not recorded.
    pub exit_code: i32,
    /// Normalized standard output.
    pub stdout: String,
    /// Normalized standard error.
    pub stderr: String,
}

/// One recorded unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
    /// The unit's key.
    pub key: DigestSet,
    /// Whether the unit was a declaration or a module.
    pub granularity: Granularity,
    /// The unit's name, for a reader of the store; no part of the key.
    pub label: String,
    /// The environment the key was computed under.
    pub environment: ReplayEnvironment,
    /// The executable that replayed it.
    pub executable: ExecutableIdentity,
    /// What it printed.
    pub result: ReplayResult,
}

/// Why a lookup found nothing to reuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Miss {
    /// No entry under the key.
    Absent,
    /// An entry that another executable produced.
    ExecutableChanged,
}

/// The outcome of a lookup that was not refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    /// A result to reuse.
    Hit(Box<CacheEntry>),
    /// Nothing to reuse.
    Miss(Miss),
}

fn rejected(path: &Utf8Path, reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(
        code!("LLV7021"),
        format!("replay cache entry `{path}` is rejected: {reason}"),
    )
}

impl CacheEntry {
    fn body(&self) -> Vec<(&'static str, Json)> {
        vec![
            ("spec", Json::Str(ENTRY_SPEC.to_owned())),
            ("key", self.key.to_json()),
            (
                "unit",
                Json::object(vec![
                    (
                        "granularity",
                        Json::Str(self.granularity.as_str().to_owned()),
                    ),
                    ("label", Json::Str(self.label.clone())),
                ]),
            ),
            (
                "environment",
                Json::object(vec![
                    ("toolchain", Json::Str(self.environment.toolchain.clone())),
                    (
                        "mathlib_revision",
                        Json::Str(self.environment.mathlib_revision.clone()),
                    ),
                ]),
            ),
            (
                "executable",
                Json::object(vec![
                    ("tool", Json::Str(self.executable.tool.clone())),
                    ("digest", self.executable.digest.to_json()),
                ]),
            ),
            (
                "result",
                Json::object(vec![
                    ("exit_code", Json::Int(i64::from(self.result.exit_code))),
                    ("stdout", Json::Str(self.result.stdout.clone())),
                    ("stderr", Json::Str(self.result.stderr.clone())),
                ]),
            ),
        ]
    }

    /// The entry's file bytes: canonical JSON whose last member is the
    /// DigestSet of the canonical string of all the others.
    #[must_use]
    pub fn to_file_bytes(&self) -> Vec<u8> {
        let without = Json::object(self.body());
        let digest = DigestSet::of(without.to_canonical_string().as_bytes());
        let Json::Obj(mut members) = without else {
            unreachable!("an object was built")
        };
        members.insert("entry_digest".to_owned(), digest.to_json());
        Json::Obj(members).to_file_bytes()
    }

    /// Read an entry's bytes, requiring every member and no other, the digest
    /// to be that of the rest, and a zero exit code. `path` only names the
    /// entry in the refusal.
    fn from_file_bytes(path: &Utf8Path, bytes: &[u8]) -> Result<Self, Diagnostic> {
        let Json::Obj(mut members) = Json::parse(bytes).map_err(|e| rejected(path, e))? else {
            return Err(rejected(path, "it is not a JSON object"));
        };
        let recorded = members
            .remove("entry_digest")
            .ok_or_else(|| rejected(path, "it has no `entry_digest`"))?;
        let recorded = DigestSet::from_json(&recorded).map_err(|e| rejected(path, e))?;
        let actual = DigestSet::of(Json::Obj(members.clone()).to_canonical_string().as_bytes());
        if recorded != actual {
            return Err(rejected(
                path,
                format!(
                    "its bytes changed: its entry digest is {} and its other members hash to {}",
                    recorded.label(),
                    actual.label()
                ),
            ));
        }
        let string_of = |value: &Json, what: &str| -> Result<String, Diagnostic> {
            match value {
                Json::Str(text) => Ok(text.clone()),
                _ => Err(rejected(path, format!("`{what}` is not a string"))),
            }
        };
        let object_of = |name: &str,
                         members: &std::collections::BTreeMap<String, Json>,
                         allowed: &[&str]|
         -> Result<std::collections::BTreeMap<String, Json>, Diagnostic> {
            match members.get(name) {
                Some(Json::Obj(inner)) => {
                    let exact = inner.len() == allowed.len()
                        && allowed.iter().all(|member| inner.contains_key(*member));
                    if exact {
                        Ok(inner.clone())
                    } else {
                        Err(rejected(
                            path,
                            format!("`{name}` does not have exactly {allowed:?}"),
                        ))
                    }
                }
                _ => Err(rejected(
                    path,
                    format!("`{name}` is missing or not an object"),
                )),
            }
        };
        let expected = ["environment", "executable", "key", "result", "spec", "unit"];
        if members.len() != expected.len() || !expected.iter().all(|m| members.contains_key(*m)) {
            return Err(rejected(
                path,
                format!("its members are not exactly {expected:?}"),
            ));
        }
        if string_of(&members["spec"], "spec")? != ENTRY_SPEC {
            return Err(rejected(
                path,
                "its `spec` is not this cache's entry format",
            ));
        }
        let key = DigestSet::from_json(&members["key"]).map_err(|e| rejected(path, e))?;
        let unit = object_of("unit", &members, &["granularity", "label"])?;
        let granularity = Granularity::parse(&string_of(&unit["granularity"], "unit.granularity")?)
            .ok_or_else(|| {
                rejected(
                    path,
                    "its granularity is neither `declaration` nor `module`",
                )
            })?;
        let environment = object_of("environment", &members, &["mathlib_revision", "toolchain"])?;
        let executable = object_of("executable", &members, &["digest", "tool"])?;
        let result = object_of("result", &members, &["exit_code", "stderr", "stdout"])?;
        if result["exit_code"] != Json::Int(0) {
            return Err(rejected(path, "it records a replay that did not exit zero"));
        }
        Ok(Self {
            key,
            granularity,
            label: string_of(&unit["label"], "unit.label")?,
            environment: ReplayEnvironment {
                toolchain: string_of(&environment["toolchain"], "environment.toolchain")?,
                mathlib_revision: string_of(
                    &environment["mathlib_revision"],
                    "environment.mathlib_revision",
                )?,
            },
            executable: ExecutableIdentity {
                tool: string_of(&executable["tool"], "executable.tool")?,
                digest: DigestSet::from_json(&executable["digest"])
                    .map_err(|e| rejected(path, e))?,
            },
            result: ReplayResult {
                exit_code: 0,
                stdout: string_of(&result["stdout"], "result.stdout")?,
                stderr: string_of(&result["stderr"], "result.stderr")?,
            },
        })
    }
}

/// A replay cache store: a directory the caller names.
#[derive(Debug, Clone)]
pub struct ReplayCache {
    root: Utf8PathBuf,
}

impl ReplayCache {
    /// A store in `root`. Nothing is read or created until a lookup or a record.
    #[must_use]
    pub fn open(root: impl Into<Utf8PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Where the entry of a key lives: `entries/<aa>/<64 hex digits>.json`,
    /// `aa` being the first two digits of the `blake3` value, which keeps a
    /// directory of a Mathlib-sized store to a few thousand names.
    #[must_use]
    pub fn entry_path(&self, key: &DigestSet) -> Utf8PathBuf {
        let hex = key.blake3.to_hex();
        self.root
            .join("entries")
            .join(&hex[..2])
            .join(format!("{hex}.json"))
    }

    /// Look a unit up. No file is a miss. A file that is not a valid entry for
    /// this key and this environment is `LLV7021`: it is neither reused nor
    /// replaced, so a store that was edited fails loudly. An entry another
    /// executable produced is a miss.
    pub fn lookup(
        &self,
        key: &DigestSet,
        environment: &ReplayEnvironment,
        executable: &ExecutableIdentity,
    ) -> Result<Lookup, Diagnostic> {
        let path = self.entry_path(key);
        let metadata = match std::fs::symlink_metadata(path.as_std_path()) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Lookup::Miss(Miss::Absent));
            }
            Err(error) => {
                return Err(Diagnostic::new(
                    code!("LLV7010"),
                    format!("cannot read the replay cache entry `{path}`: {error}"),
                ));
            }
        };
        if !metadata.is_file() {
            return Err(rejected(&path, "it is not a regular file"));
        }
        let bytes = std::fs::read(path.as_std_path()).map_err(|error| {
            Diagnostic::new(
                code!("LLV7010"),
                format!("cannot read the replay cache entry `{path}`: {error}"),
            )
        })?;
        let entry = CacheEntry::from_file_bytes(&path, &bytes)?;
        if entry.key != *key {
            return Err(rejected(
                &path,
                format!(
                    "it records the key {} and was looked up under {}",
                    entry.key.label(),
                    key.label()
                ),
            ));
        }
        if entry.environment != *environment {
            return Err(rejected(
                &path,
                "its environment is not the one its key covers",
            ));
        }
        if entry.executable != *executable {
            return Ok(Lookup::Miss(Miss::ExecutableChanged));
        }
        Ok(Lookup::Hit(Box::new(entry)))
    }

    /// Record a unit: a temporary file in the entry's directory, renamed into
    /// place, so a reader sees a whole entry or none.
    pub fn record(&self, entry: &CacheEntry) -> Result<(), Diagnostic> {
        let path = self.entry_path(&entry.key);
        let io_error = |error: std::io::Error| {
            Diagnostic::new(
                code!("LLV7010"),
                format!("cannot write the replay cache entry `{path}`: {error}"),
            )
        };
        let directory = path.parent().expect("an entry path has a parent");
        std::fs::create_dir_all(directory.as_std_path()).map_err(io_error)?;
        let staged = directory.join(format!(
            ".{}.tmp{}",
            entry.key.blake3.to_hex(),
            std::process::id()
        ));
        std::fs::write(staged.as_std_path(), entry.to_file_bytes()).map_err(io_error)?;
        std::fs::rename(staged.as_std_path(), path.as_std_path()).map_err(io_error)?;
        crate::artifact::fsync_dir(directory.as_std_path());
        Ok(())
    }
}
