use super::{dependency_roles, gather, SKIP_DIRS};

#[test]
fn source_audits_exclude_only_repository_relative_generated_paths() {
    let temporary = tempfile::tempdir().expect("temporary repository");
    for ancestor in SKIP_DIRS {
        for basename in ["repository", ancestor] {
            let root = temporary.path().join(ancestor).join(basename);
            std::fs::create_dir_all(root.join("crates/source")).unwrap();
            std::fs::write(root.join("crates/source/main.rs"), "source").unwrap();
            std::fs::write(root.join("crates/source/other.txt"), "not selected").unwrap();
            std::fs::write(root.join("README.md"), "root document").unwrap();
            for generated in SKIP_DIRS {
                let nested = root.join("crates/source").join(generated);
                std::fs::create_dir_all(&nested).unwrap();
                std::fs::write(nested.join("generated.rs"), "generated").unwrap();
            }
            let mut selected = Vec::new();
            gather(&root, &["crates"], &[".rs"], &mut selected);
            assert_eq!(
                selected,
                [root.join("README.md"), root.join("crates/source/main.rs")],
                "ancestor {ancestor}, basename {basename}"
            );
        }
    }
}

const MANIFEST: &str = r#"
[dependencies]
camino = { workspace = true }
fs4 = { workspace = true }

[target.'cfg(unix)'.dependencies]
rustix = { workspace = true }

[dev-dependencies]
proptest = { workspace = true }
"#;

const SPEC: &str = "\n### 8.5 Required implementation dependencies\n\n| Dependency | Role |\n| --- | --- |\n| `camino` | UTF-8 path handling |\n| `fs4` | file locking |\n| `rustix` | process-group signalling |\n\n---\n\n## 9. Root tooling\n| `unrelated` | not in the section |\n";

#[test]
fn every_shipped_dependency_has_a_role_and_every_role_a_dependency() {
    assert_eq!(dependency_roles(MANIFEST, SPEC).expect("agree"), 3);
    // A dependency of any target and no role: refused, by name.
    let added = MANIFEST.replace(
        "[dev-dependencies]",
        "[target.'cfg(windows)'.dependencies]\nwinapi = \"0.3\"\n\n[dev-dependencies]",
    );
    let error = dependency_roles(&added, SPEC).expect_err("no role");
    assert!(error.to_string().contains("`winapi`"), "{error}");
    // A role and no dependency: refused, by name.
    let removed = MANIFEST.replace("fs4 = { workspace = true }\n", "");
    let error = dependency_roles(&removed, SPEC).expect_err("no dependency");
    assert!(error.to_string().contains("`fs4`"), "{error}");
    // A development dependency needs no role.
    assert!(!SPEC.contains("proptest"));
    // Not vacuous: a SPEC without the table is refused, not passed.
    let error = dependency_roles(MANIFEST, "no section").expect_err("no table");
    assert!(error.to_string().contains("no table"), "{error}");
}
