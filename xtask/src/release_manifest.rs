//! Release inventories describe captured bytes, never mutable Cargo caches.

use crate::Fail;
use lexlean::artifact::content_id::Sha256Digest;
use std::collections::BTreeMap;
use std::io::{Cursor, Read, Write};
use std::path::{Component, Path, PathBuf};

fn canonical_name(name: &str) -> Result<(), Fail> {
    if name.is_empty()
        || name.contains(['\\', ':'])
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(format!("RP-12: noncanonical release member {name:?}").into());
    }
    Ok(())
}

fn render(files: BTreeMap<String, String>) -> Result<String, Fail> {
    if files.is_empty() {
        return Err("RP-12: release inventory has no regular files".into());
    }
    Ok(files
        .into_iter()
        .map(|(name, digest)| format!("{digest}  {name}\n"))
        .collect())
}

pub(super) fn crate_manifest(bytes: &[u8], version: &str) -> Result<String, Fail> {
    let prefix = format!("lexlean-{version}");
    canonical_name(&prefix)?;
    let mut decoder = flate2::bufread::GzDecoder::new(bytes);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded)?;
    if !decoder.into_inner().is_empty() {
        return Err("RP-12: data follows the crate gzip member".into());
    }
    if decoded.len() < 1024
        || decoded.len() % 512 != 0
        || decoded[decoded.len() - 1024..]
            .iter()
            .any(|byte| *byte != 0)
    {
        return Err("RP-12: crate tar terminator is incomplete".into());
    }
    let mut archive = tar::Archive::new(Cursor::new(decoded.as_slice()));
    let mut members = BTreeMap::new();
    let mut files = BTreeMap::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        let kind = entry.header().entry_type();
        if !kind.is_file() && !kind.is_dir() {
            return Err("RP-12: crate members must be regular files or directories".into());
        }
        if entry.link_name_bytes().is_some() {
            return Err("RP-12: crate member carries a link target".into());
        }
        let path_bytes = entry.path_bytes();
        let raw_name = std::str::from_utf8(&path_bytes)?;
        let name = if kind.is_dir() {
            raw_name.strip_suffix('/').unwrap_or(raw_name)
        } else {
            raw_name
        };
        canonical_name(name)?;
        if name != prefix && !name.starts_with(&(prefix.clone() + "/")) {
            return Err("RP-12: crate member is outside its exact package root".into());
        }
        if name == prefix && !kind.is_dir() {
            return Err("RP-12: crate root must be a directory".into());
        }
        let name = name.to_owned();
        if members.insert(name.clone(), kind.is_file()).is_some() {
            return Err("RP-12: duplicate crate member".into());
        }
        if kind.is_file() {
            let relative = name
                .strip_prefix(&(prefix.clone() + "/"))
                .ok_or("crate root has no member")?;
            let mut content = Vec::new();
            entry.read_to_end(&mut content)?;
            if content.len() as u64 != entry.size() {
                return Err("RP-12: incomplete crate member".into());
            }
            files.insert(relative.to_owned(), Sha256Digest::of(&content).to_hex());
        } else if entry.size() != 0 {
            return Err("RP-12: crate directory contains payload bytes".into());
        }
    }
    let consumed = usize::try_from(archive.into_inner().position())?;
    if decoded[consumed..].iter().any(|byte| *byte != 0) {
        return Err("RP-12: data follows the crate tar terminator".into());
    }
    for name in members.keys() {
        for (at, _) in name.match_indices('/') {
            if members.get(&name[..at]) == Some(&true) {
                return Err("RP-12: crate member is nested under a regular file".into());
            }
        }
    }
    if !files.contains_key("Cargo.toml") {
        return Err("RP-12: crate has no Cargo.toml".into());
    }
    render(files)
}

pub(super) fn tree_files(root: &Path) -> Result<BTreeMap<String, PathBuf>, Fail> {
    let canonical = directory_root(root)?;
    let root = canonical.as_path();
    let mut files = BTreeMap::new();
    for entry in walkdir::WalkDir::new(root).follow_links(false) {
        let entry = entry?;
        reject_alias(&std::fs::symlink_metadata(entry.path())?)?;
        if entry.file_type().is_dir() {
            continue;
        }
        if !entry.file_type().is_file() {
            return Err("RP-12: release tree contains a link or special file".into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if entry.metadata()?.nlink() != 1 {
                return Err("RP-12: release file has multiple hard links".into());
            }
        }
        let parts = entry
            .path()
            .strip_prefix(root)?
            .components()
            .map(|component| {
                let Component::Normal(part) = component else {
                    return Err("RP-12: release path is not relative".into());
                };
                let part = part.to_str().ok_or("RP-12: release path is not UTF-8")?;
                canonical_name(part)?;
                Ok(part.to_owned())
            })
            .collect::<Result<Vec<_>, Fail>>()?;
        let name = parts.join("/");
        canonical_name(&name)?;
        if files.insert(name, entry.path().to_path_buf()).is_some() {
            return Err("RP-12: duplicate normalized release path".into());
        }
    }
    Ok(files)
}

fn reject_alias(metadata: &std::fs::Metadata) -> Result<(), Fail> {
    if metadata.file_type().is_symlink() {
        return Err("RP-12: release paths cannot contain symbolic links".into());
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        // Includes junctions, not just the symbolic-link reparse tag.
        if metadata.file_attributes() & 0x400 != 0 {
            return Err("RP-12: release paths cannot contain reparse points".into());
        }
    }
    Ok(())
}

pub(super) fn directory_root(root: &Path) -> Result<PathBuf, Fail> {
    if !root.is_absolute() {
        return Err("RP-12: release root must be absolute".into());
    }
    for ancestor in root.ancestors() {
        let metadata = std::fs::symlink_metadata(ancestor)?;
        reject_alias(&metadata)?;
        if !metadata.is_dir() {
            return Err("RP-12: release ancestor is not a directory".into());
        }
    }
    // Windows adds a verbatim prefix; textual inequality is not alias proof.
    Ok(root.canonicalize()?)
}

pub(super) fn write_asset(root: &Path, name: &str, bytes: &[u8]) -> Result<(), Fail> {
    canonical_name(name)?;
    if name.contains('/') {
        return Err("RP-12: derived release assets require a flat name".into());
    }
    let root = directory_root(root)?;
    let mut staged = tempfile::NamedTempFile::new_in(&root)?;
    staged.write_all(bytes)?;
    staged.as_file().sync_all()?;
    // Replace the directory entry, never truncate a possibly hard-linked inode.
    staged.persist(root.join(name))?;
    Ok(())
}

pub(super) fn tree_manifest(root: &Path, excluded: &str) -> Result<String, Fail> {
    canonical_name(excluded)?;
    let mut rows = BTreeMap::new();
    for (name, path) in tree_files(root)? {
        if name != excluded {
            rows.insert(name, Sha256Digest::of(&std::fs::read(path)?).to_hex());
        }
    }
    render(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(entries: &[(&str, tar::EntryType, &[u8])]) -> Vec<u8> {
        let mut tar = tar::Builder::new(Vec::new());
        for (path, kind, content) in entries {
            let mut header = tar::Header::new_gnu();
            header.as_mut_bytes()[..path.len()].copy_from_slice(path.as_bytes());
            header.set_entry_type(*kind);
            header.set_size(content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append(&header, *content).unwrap();
        }
        gzip(&tar.into_inner().unwrap())
    }

    fn gzip(bytes: &[u8]) -> Vec<u8> {
        let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        gzip.write_all(bytes).unwrap();
        gzip.finish().unwrap()
    }

    #[test]
    fn manifest_hashes_exact_archive_not_the_unpacked_verification_tree() {
        let regular = tar::EntryType::Regular;
        let bytes = package(&[
            ("lexlean-0.3.0/src/lib.rs", regular, b"source"),
            ("lexlean-0.3.0/Cargo.toml", regular, b"manifest"),
        ]);
        let stale = tempfile::tempdir().unwrap();
        std::fs::write(stale.path().join("Cargo.toml"), b"stale").unwrap();
        std::fs::write(stale.path().join("extra.rs"), b"not packaged").unwrap();
        let expected = format!(
            "{}  Cargo.toml\n{}  src/lib.rs\n",
            Sha256Digest::of(b"manifest").to_hex(),
            Sha256Digest::of(b"source").to_hex()
        );
        assert_eq!(crate_manifest(&bytes, "0.3.0").unwrap(), expected);
        std::fs::remove_file(stale.path().join("Cargo.toml")).unwrap();
        assert_eq!(crate_manifest(&bytes, "0.3.0").unwrap(), expected);
        assert!(crate_manifest(&bytes, "0.4.0").is_err());
    }

    #[test]
    fn archive_inventory_refuses_aliases_duplicates_special_files_and_missing_members() {
        let manifest = (
            "lexlean-0.3.0/Cargo.toml",
            tar::EntryType::Regular,
            &b"manifest"[..],
        );
        for name in [
            "other/src.rs",
            "lexlean-0.3.0/../escape",
            "lexlean-0.3.0/./alias",
            "/lexlean-0.3.0/absolute",
            "lexlean-0.3.0/a//b",
            "lexlean-0.3.0/back\\slash",
            "lexlean-0.3.0/drive:alias",
            "lexlean-0.3.0/bad\nname",
        ] {
            assert!(
                crate_manifest(
                    &package(&[manifest, (name, tar::EntryType::Regular, b"x")]),
                    "0.3.0"
                )
                .is_err(),
                "{name:?}"
            );
        }
        for kind in [
            tar::EntryType::Symlink,
            tar::EntryType::Link,
            tar::EntryType::Fifo,
            tar::EntryType::Char,
            tar::EntryType::Block,
        ] {
            assert!(crate_manifest(
                &package(&[manifest, ("lexlean-0.3.0/link", kind, b"")]),
                "0.3.0"
            )
            .is_err());
        }
        assert!(crate_manifest(&package(&[manifest, manifest]), "0.3.0").is_err());
        assert!(crate_manifest(
            &package(&[
                manifest,
                (
                    "lexlean-0.3.0/Cargo.toml/nested",
                    tar::EntryType::Regular,
                    b"x"
                )
            ]),
            "0.3.0"
        )
        .is_err());
        assert!(crate_manifest(&package(&[]), "0.3.0").is_err());
        assert!(crate_manifest(
            &package(&[("lexlean-0.3.0/src.rs", tar::EntryType::Regular, b"x")]),
            "0.3.0"
        )
        .is_err());
    }

    #[test]
    fn archive_inventory_refuses_truncation_corruption_and_trailing_archives() {
        let bytes = package(&[(
            "lexlean-0.3.0/Cargo.toml",
            tar::EntryType::Regular,
            b"manifest",
        )]);
        for at in [0, 1, bytes.len() / 2, bytes.len() - 1] {
            assert!(crate_manifest(&bytes[..at], "0.3.0").is_err());
        }
        let mut corrupt = bytes.clone();
        let at = corrupt.len() - 8;
        corrupt[at] ^= 1;
        assert!(crate_manifest(&corrupt, "0.3.0").is_err());
        assert!(crate_manifest(&[bytes.as_slice(), bytes.as_slice()].concat(), "0.3.0").is_err());
        let mut raw = Vec::new();
        flate2::read::GzDecoder::new(bytes.as_slice())
            .read_to_end(&mut raw)
            .unwrap();
        let twice = [raw.as_slice(), raw.as_slice()].concat();
        assert!(crate_manifest(&gzip(&twice), "0.3.0").is_err());
    }

    #[test]
    fn legal_long_member_names_use_the_effective_gnu_and_pax_paths() {
        let relative = format!("{}/{}/source.rs", "a".repeat(80), "b".repeat(80));
        let path = format!("lexlean-0.3.0/{relative}");
        for pax in [false, true] {
            let mut tar = tar::Builder::new(Vec::new());
            let mut header = tar::Header::new_gnu();
            header.set_mode(0o644);
            header.set_size(8);
            header.set_cksum();
            tar.append_data(&mut header, "lexlean-0.3.0/Cargo.toml", &b"manifest"[..])
                .unwrap();
            if pax {
                tar.append_pax_extensions([("path", path.as_bytes())])
                    .unwrap();
            }
            header.set_size(6);
            header.set_cksum();
            tar.append_data(
                &mut header,
                if pax {
                    "ignored-short-name"
                } else {
                    path.as_str()
                },
                &b"source"[..],
            )
            .unwrap();
            let expected = format!(
                "{}  Cargo.toml\n{}  {relative}\n",
                Sha256Digest::of(b"manifest").to_hex(),
                Sha256Digest::of(b"source").to_hex()
            );
            assert_eq!(
                crate_manifest(&gzip(&tar.into_inner().unwrap()), "0.3.0").unwrap(),
                expected
            );
        }
    }

    #[test]
    fn tree_inventory_is_complete_sorted_and_propagates_failures() {
        let work = tempfile::tempdir().unwrap();
        let root = work.path().canonicalize().unwrap();
        std::fs::write(root.join("z"), b"last").unwrap();
        std::fs::write(root.join("a"), b"first").unwrap();
        std::fs::write(root.join("checksums.txt"), b"old").unwrap();
        let expected = format!(
            "{}  a\n{}  z\n",
            Sha256Digest::of(b"first").to_hex(),
            Sha256Digest::of(b"last").to_hex()
        );
        assert_eq!(tree_manifest(&root, "checksums.txt").unwrap(), expected);
        assert!(tree_manifest(&root.join("missing"), "checksums.txt").is_err());
        // Some supported filesystems reject control-bearing names themselves.
        // The archive tests above exercise this name rejection on every host.
        if std::fs::write(root.join("bad\nname"), b"bad").is_ok() {
            assert!(tree_manifest(&root, "checksums.txt").is_err());
        } else {
            eprintln!("filesystem refuses a newline-bearing fixture name");
        }
    }

    #[test]
    fn normal_root_spelling_and_atomic_asset_replacement_preserve_external_links() {
        let work = tempfile::tempdir().unwrap();
        let canonical = work.path().canonicalize().unwrap();
        // On Windows this removes the canonical verbatim prefix, as repo_root
        // obtains an ordinary CARGO_MANIFEST_DIR path on that host.
        #[cfg(windows)]
        let ordinary = {
            let text = canonical.to_str().unwrap();
            PathBuf::from(if let Some(path) = text.strip_prefix("\\\\?\\UNC\\") {
                format!("\\\\{path}")
            } else {
                text.strip_prefix("\\\\?\\").unwrap_or(text).to_owned()
            })
        };
        #[cfg(not(windows))]
        let ordinary = canonical.clone();
        assert_eq!(directory_root(&ordinary).unwrap(), canonical);
        let external = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(external.path(), b"must remain unchanged").unwrap();
        std::fs::hard_link(external.path(), ordinary.join("asset")).unwrap();
        write_asset(&ordinary, "asset", b"published").unwrap();
        assert_eq!(
            std::fs::read(external.path()).unwrap(),
            b"must remain unchanged"
        );
        assert_eq!(std::fs::read(ordinary.join("asset")).unwrap(), b"published");
        assert_eq!(tree_files(&ordinary).unwrap().len(), 1);
        assert!(write_asset(&ordinary, "../outside", b"bad").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn tree_inventory_rejects_symlinks_hardlinks_and_unreadable_directories() {
        use std::os::unix::fs::{symlink, PermissionsExt};
        let work = tempfile::tempdir().unwrap();
        let root = work.path().canonicalize().unwrap();
        let linked_root = root.join("linked-root");
        std::fs::create_dir(root.join("child")).unwrap();
        symlink(&root, &linked_root).unwrap();
        assert!(directory_root(&linked_root).is_err());
        assert!(directory_root(&linked_root.join("child")).is_err());
        std::fs::remove_file(&linked_root).unwrap();
        std::fs::write(root.join("file"), b"file").unwrap();
        symlink("file", root.join("alias")).unwrap();
        assert!(tree_manifest(&root, "checksums.txt").is_err());
        std::fs::remove_file(root.join("alias")).unwrap();
        std::fs::hard_link(root.join("file"), root.join("alias")).unwrap();
        assert!(tree_manifest(&root, "checksums.txt").is_err());
        std::fs::remove_file(root.join("alias")).unwrap();
        let hidden = root.join("hidden");
        std::fs::create_dir(&hidden).unwrap();
        std::fs::write(hidden.join("file"), b"must not disappear").unwrap();
        std::fs::set_permissions(&hidden, std::fs::Permissions::from_mode(0o0)).unwrap();
        let inaccessible = std::fs::read_dir(&hidden).is_err();
        let result = tree_manifest(&root, "checksums.txt");
        std::fs::set_permissions(&hidden, std::fs::Permissions::from_mode(0o700)).unwrap();
        if inaccessible {
            assert!(result.is_err());
        } else {
            eprintln!("permission refusal cannot be provoked by this privileged filesystem user");
        }
        assert_eq!(
            tree_manifest(&root, "checksums.txt")
                .unwrap()
                .lines()
                .count(),
            2
        );
    }
}
