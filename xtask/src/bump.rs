//! `cargo xtask bump-version <X.Y.Z>`: the hand-held sites of the workspace
//! version (SPEC.md §2.3) moved together, for a release cut (§9.4, `cut.yml`).
//!
//! The version is fixed in the workspace manifest, repeated by the §2.3 block
//! of SPEC.md and the RP-01 identity case that holds the two equal, and named
//! by `## Unreleased` in CHANGELOG.md until a release takes it. A cut moved each
//! by hand; this moves all of them or none, and refuses when a site does not
//! read as it expects, so a site that drifted is named rather than skipped.
//! What the version moves downstream (the lock's workspace rows, and every
//! build oracle's `compiler.version`) is written by `cargo update --workspace`
//! here and by the write recipes of `regenerate.yml` after it.

use std::path::Path;

use crate::Fail;

/// One rewrite: the file, the text it must hold `count` times at the old
/// version, and that text at the new one.
struct Site {
    path: &'static str,
    old: String,
    new: String,
    count: usize,
}

fn parse(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.');
    let mut next = || -> Option<u64> {
        let part = parts.next()?;
        // SemVer: no leading zero, digits only.
        if part.is_empty() || (part.len() > 1 && part.starts_with('0')) {
            return None;
        }
        if !part.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        part.parse().ok()
    };
    let parsed = (next()?, next()?, next()?);
    parts.next().is_none().then_some(parsed)
}

fn sites(old: &str, new: &str) -> Vec<Site> {
    let site = |path, old: String, new: String, count| Site {
        path,
        old,
        new,
        count,
    };
    vec![
        // `[workspace.package]` and the three path dependencies of the workspace.
        site(
            "Cargo.toml",
            format!("version = \"{old}\""),
            format!("version = \"{new}\""),
            4,
        ),
        site(
            "SPEC.md",
            format!("[workspace.package]\nversion = \"{old}\"\n"),
            format!("[workspace.package]\nversion = \"{new}\"\n"),
            1,
        ),
        site(
            "crates/conformance/src/cases/repository.rs",
            format!("\"version = \\\"{old}\\\"\","),
            format!("\"version = \\\"{new}\\\"\","),
            1,
        ),
        site(
            "crates/conformance/src/cases/repository.rs",
            format!("stdout.starts_with(\"lexlean {old}\\n\")"),
            format!("stdout.starts_with(\"lexlean {new}\\n\")"),
            1,
        ),
        site(
            "crates/conformance/src/cases/repository.rs",
            format!("\"a {old} tree must refuse release"),
            format!("\"a {new} tree must refuse release"),
            1,
        ),
        // The unreleased entries become the release's (Keep a Changelog); the
        // next landing opens a new `## Unreleased`.
        site(
            "CHANGELOG.md",
            "\n## Unreleased\n".to_owned(),
            format!("\n## {new}\n"),
            1,
        ),
    ]
}

/// Move every site from the workspace version to `new`, then the lock's
/// workspace rows. Nothing is written unless every site reads as expected.
pub fn bump_version(root: &Path, arguments: &[String]) -> Result<(), Fail> {
    let [new] = arguments else {
        return Err("cargo xtask bump-version <X.Y.Z>".into());
    };
    let old = crate::workspace_version(root)?;
    let (Some(from), Some(to)) = (parse(&old), parse(new)) else {
        return Err(
            format!("§2.3: `{old}` to `{new}` is not a move between X.Y.Z versions").into(),
        );
    };
    if to <= from {
        return Err(format!("§2.3: `{new}` does not follow `{old}`").into());
    }
    let mut texts: Vec<(&'static str, String)> = Vec::new();
    let mut unmet = Vec::new();
    for site in sites(&old, new) {
        let index = match texts.iter().position(|(path, _)| *path == site.path) {
            Some(index) => index,
            None => {
                texts.push((site.path, std::fs::read_to_string(root.join(site.path))?));
                texts.len() - 1
            }
        };
        let text = &mut texts[index].1;
        let found = text.matches(&site.old).count();
        if found == site.count {
            *text = text.replace(&site.old, &site.new);
        } else {
            unmet.push(format!(
                "{}: {:?} occurs {found} times, not {}",
                site.path, site.old, site.count
            ));
        }
    }
    if !unmet.is_empty() {
        return Err(format!(
            "bump-version {old} -> {new} wrote nothing; the sites do not read as expected:\n  {}",
            unmet.join("\n  ")
        )
        .into());
    }
    for (path, text) in &texts {
        std::fs::write(root.join(path), text)?;
    }
    let status =
        std::process::Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned()))
            .current_dir(root)
            .args(["update", "--workspace", "--offline"])
            .status()?;
    if !status.success() {
        return Err("cargo update --workspace --offline failed after the version moved".into());
    }
    println!(
        "bump-version: {old} -> {new} in {} files and Cargo.lock",
        texts.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn versions_are_three_plain_numbers() {
        assert_eq!(parse("0.4.1"), Some((0, 4, 1)));
        assert_eq!(parse("1.0.0"), Some((1, 0, 0)));
        for refused in [
            "0.4",
            "0.4.1.2",
            "v0.4.1",
            "0.04.1",
            "0.4.1-rc.1",
            "0.4.x",
            "",
        ] {
            assert_eq!(parse(refused), None, "{refused}");
        }
        assert!(parse("0.5.0") > parse("0.4.9"));
    }
}
