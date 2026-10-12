//! A PrismPM-style consumer of the production export contract (SPEC.md
//! §24.7, PX-01). It names only LexLean's stable surface: the crate-root
//! types of §24 and `lexlean::api::production`. `conformance_px_01` reads this
//! file and refuses any path into a hidden module, so the fixture cannot pass
//! by reaching into the compiler.
//!
//! The application's own description of its roots is the consumer's format,
//! not LexLean's: which roots it ships, under which crate names, in which
//! targets, with which certificate evidence.

use std::collections::BTreeMap;

use camino::Utf8Path;
use lexlean::api::production::{
    self, Evidence, EvidenceKind, Expectation, PackageName, ProductionOutput, ProductionRequest,
    ReplayReport, RootRequest, TargetRef, REQUEST_SPEC,
};
use lexlean::{CheckRequest, Engine, LexLeanError, LockRequest, Selection, Sha256Digest};
use serde::Deserialize;

/// The application's roots (`roots.json` beside its expected packages).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Application {
    /// The roots it ships.
    pub roots: Vec<AppRoot>,
}

/// One shipped root.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppRoot {
    /// The qualified root name.
    pub root: String,
    /// The crate name.
    pub name: String,
    /// The crate version.
    pub version: String,
    /// The target IDs it ships in.
    pub targets: Vec<String>,
}

/// GNAF evidence the application offers for one root.
#[derive(Debug, Clone)]
pub struct AppEvidence {
    /// The root.
    pub root: String,
    /// The project-relative request path.
    pub request: String,
    /// The project-relative certificate path.
    pub certificate: String,
    /// The certificate identity the application means.
    pub certificate_id: Sha256Digest,
}

/// What setup pins: the project's identities and the target digests.
#[derive(Debug, Clone)]
pub struct Pins {
    /// The pinned identities.
    pub expect: Expectation,
    /// Target ID to descriptor digest.
    pub targets: BTreeMap<String, Sha256Digest>,
}

/// Setup: the one step allowed to touch more than the project. Here the lock
/// is only checked, never acquired, so even setup is offline.
///
/// # Errors
///
/// Any registered failure of `lock`, `check`, or `targets`.
pub fn setup(project_file: &Utf8Path) -> Result<(Engine, Pins), LexLeanError> {
    let engine = Engine::load(project_file)?;
    let lock = engine.lock(LockRequest {
        check_only: true,
        allow_network: false,
    })?;
    let checked = engine.check(CheckRequest {
        selection: Selection::Entrypoints,
    })?;
    let targets = production::targets()?
        .into_iter()
        .map(|descriptor| (descriptor.id, descriptor.digest))
        .collect();
    Ok((
        engine,
        Pins {
            expect: Expectation {
                source_id: Some(checked.source_id),
                semantic_id: Some(checked.semantic_id),
                lock_sha256: Some(Sha256Digest::of(&lock.bytes)),
            },
            targets,
        },
    ))
}

/// The request for an application's roots under the pins of setup.
#[must_use]
pub fn request(
    application: &Application,
    pins: &Pins,
    evidence: &[AppEvidence],
) -> ProductionRequest {
    let mut roots: Vec<RootRequest> = application
        .roots
        .iter()
        .map(|root| RootRequest {
            root: root.root.clone(),
            package: PackageName {
                name: root.name.clone(),
                version: root.version.clone(),
            },
            targets: root
                .targets
                .iter()
                .map(|id| TargetRef {
                    id: id.clone(),
                    digest: pins
                        .targets
                        .get(id)
                        .copied()
                        .unwrap_or(Sha256Digest([0; 32])),
                })
                .collect(),
        })
        .collect();
    roots.sort_by(|left, right| left.root.cmp(&right.root));
    let mut offered: Vec<Evidence> = evidence
        .iter()
        .map(|offer| Evidence {
            root: offer.root.clone(),
            kind: EvidenceKind::Gnaf,
            request: offer.request.clone(),
            certificate: offer.certificate.clone(),
            certificate_id: offer.certificate_id,
        })
        .collect();
    offered.sort_by(|left, right| left.root.cmp(&right.root));
    ProductionRequest {
        spec: REQUEST_SPEC.to_owned(),
        roots,
        expect: pins.expect.clone(),
        evidence: offered,
    }
}

/// Setup, export, and replay the published bundle.
///
/// # Errors
///
/// Any registered failure of the contract.
pub fn export(
    project_file: &Utf8Path,
    application: &Application,
    evidence: &[AppEvidence],
) -> Result<(ProductionOutput, ReplayReport), LexLeanError> {
    let (engine, pins) = setup(project_file)?;
    let request = request(application, &pins, evidence);
    let output = production::build(&engine, &request)?;
    let project_root = project_file.parent().unwrap_or(project_file);
    let report = production::verify_artifact(&project_root.join(&output.bundle))?;
    Ok((output, report))
}
