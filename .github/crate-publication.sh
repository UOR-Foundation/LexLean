#!/usr/bin/env bash
# The decisions of publishing the lexlean crate to crates.io (RP-18, #260), kept
# in one script so that each can be run against planted inputs. release.yml
# calls it; nothing here uploads, and nothing here reads or prints a token.
#
#   crate-publication.sh identity RELEASE_DIR PACKAGED
#       PACKAGED, the crate `cargo publish` would upload, must have the SHA-256
#       of RELEASE_DIR/lexlean.crate and the SHA-256 that RELEASE_DIR/checksums.txt
#       lists for it. Prints the digest.
#   crate-publication.sh metadata PACKAGED...
#       What crates.io needs from an archive: description, license, repository,
#       a readme the archive carries, and at most 10 MiB.
#   crate-publication.sh mode
#       How the upload is authorised, from HAS_CARGO_REGISTRY_TOKEN and
#       TRUSTED_PUBLISHING (each `true` or anything else): token, trusted, or none.
#   crate-publication.sh prerelease REF COUNT
#       The version of a commit of a release line: `release/v0.4` and a count of
#       first-parent commits give `0.4.0-dev.<count>`.
#   crate-publication.sh stamp VERSION [ROOT]
#       Rewrite the workspace version, and the version of the workspace's own
#       crates, in ROOT/Cargo.toml (a throwaway checkout, never a commit).
set -euo pipefail

fail() {
    printf '%s\n' "$1" >&2
    exit 1
}

digest() {
    sha256sum "$1" | cut -d' ' -f1
}

# The tag a pre-release names must be a SemVer pre-release of a release line.
semver_prerelease='^[0-9]+\.[0-9]+\.[0-9]+-dev\.[0-9]+$'
semver_release='^[0-9]+\.[0-9]+\.[0-9]+$'

identity() {
    release=$1 packaged=$2
    [[ -f "$release/lexlean.crate" ]] || fail "identity: $release/lexlean.crate is missing"
    [[ -f "$release/checksums.txt" ]] || fail "identity: $release/checksums.txt is missing"
    [[ -f "$packaged" ]] || fail "identity: $packaged is missing"
    actual=$(digest "$packaged")
    staged=$(digest "$release/lexlean.crate")
    [[ "$actual" == "$staged" ]] ||
        fail "identity: cargo would upload $actual, and the release attaches lexlean.crate $staged"
    listed=$(sed -n 's/^\([0-9a-f]\{64\}\)  lexlean\.crate$/\1/p' "$release/checksums.txt")
    [[ -n "$listed" ]] || fail "identity: checksums.txt lists no lexlean.crate"
    [[ "$(printf '%s\n' "$listed" | wc -l)" -eq 1 ]] || fail "identity: checksums.txt lists lexlean.crate twice"
    [[ "$listed" == "$actual" ]] ||
        fail "identity: cargo would upload $actual, and checksums.txt lists $listed for lexlean.crate"
    printf '%s\n' "$actual"
}

metadata() {
    limit=10485760
    status=0
    for archive in "$@"; do
        if [[ ! -f "$archive" ]]; then
            printf '%s: no such archive\n' "$archive" >&2
            status=1
            continue
        fi
        size=$(wc -c < "$archive" | tr -d ' ')
        if ((size > limit)); then
            printf '%s: %s bytes exceeds the crates.io limit of %s\n' "$archive" "$size" "$limit" >&2
            status=1
        fi
        manifest=$(tar -xzOf "$archive" --wildcards '*/Cargo.toml' 2>/dev/null || true)
        if [[ -z "$manifest" ]]; then
            printf '%s: no Cargo.toml in the archive\n' "$archive" >&2
            status=1
            continue
        fi
        package=$(printf '%s\n' "$manifest" | sed -n '/^\[package\]/,/^\[/p')
        field() {
            printf '%s\n' "$package" | sed -n "s/^$1 = \"\\(.*\\)\"\$/\\1/p" | head -1
        }
        for required in description repository; do
            if [[ -z "$(field "$required")" ]]; then
                printf '%s: Cargo.toml has no `%s`\n' "$archive" "$required" >&2
                status=1
            fi
        done
        if [[ -z "$(field license)" && -z "$(field license-file)" ]]; then
            printf '%s: Cargo.toml has neither `license` nor `license-file`\n' "$archive" >&2
            status=1
        fi
        readme=$(field readme)
        # The listing is read whole: `grep -q` leaves a pipe early, and pipefail
        # would then call a present readme missing.
        listing=$(tar -tzf "$archive")
        if [[ -n "$readme" ]] && ! grep -q -- "/$readme\$" <<<"$listing"; then
            printf '%s: `readme = "%s"` names a file the archive does not carry\n' "$archive" "$readme" >&2
            status=1
        fi
    done
    return "$status"
}

mode() {
    if [[ "${HAS_CARGO_REGISTRY_TOKEN:-}" == true ]]; then
        echo token
    elif [[ "${TRUSTED_PUBLISHING:-}" == true ]]; then
        echo trusted
    else
        echo none
    fi
}

prerelease() {
    ref=$1 count=$2
    [[ "$ref" =~ ^release/v([0-9]+)\.([0-9]+)$ ]] ||
        fail "prerelease: $ref is not a release line (release/vX.Y)"
    major=${BASH_REMATCH[1]} minor=${BASH_REMATCH[2]}
    [[ "$count" =~ ^[0-9]+$ ]] || fail "prerelease: the commit count $count is not a number"
    printf '%s.%s.0-dev.%s\n' "$major" "$minor" "$count"
}

stamp() {
    version=$1 root=${2:-.}
    [[ "$version" =~ $semver_prerelease || "$version" =~ $semver_release ]] ||
        fail "stamp: $version is not a SemVer version"
    manifest=$root/Cargo.toml
    current=$(sed -n '/^\[workspace\.package\]/,/^\[/s/^version = "\(.*\)"$/\1/p' "$manifest" | head -1)
    [[ -n "$current" ]] || fail "stamp: $manifest has no workspace version"
    escaped=${current//./\\.}
    sed -i \
        -e "/^\\[workspace\\.package\\]/,/^\\[/s/^version = \"$escaped\"\$/version = \"$version\"/" \
        -e "s/^\\(repo-model\\|repo-conformance\\|lexlean\\) = { version = \"$escaped\", path/\\1 = { version = \"$version\", path/" \
        "$manifest"
    # The workspace version and its three crates, and nothing at the old version.
    stamped=$(grep -c "version = \"$version\"" "$manifest" || true)
    [[ "$stamped" -eq 4 ]] || fail "stamp: expected 4 versions at $version in $manifest, found $stamped"
    printf '%s\n' "$current"
}

# The checkout the crate is packaged from. For a pre-release it is a throwaway
# copy of a release line at one commit: the commit count and the line name give
# the version, and the stamped manifest and lock are never committed.
prepare() {
    pre=$1
    if [[ "$pre" == true ]]; then
        [[ "$(git rev-parse --is-shallow-repository)" == false ]] ||
            fail "prepare: the commit count needs the whole history (fetch-depth: 0)"
        version=$(prerelease "${GITHUB_REF_NAME:?prepare: GITHUB_REF_NAME names the release line}" \
            "$(git rev-list --first-parent --count HEAD)")
        stamp "$version" . > /dev/null
        cargo update --workspace --quiet
        printf '%s\n' "$version"
    else
        sed -n '/^\[workspace\.package\]/,/^\[/s/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1
    fi
}

command=${1:-}
case "$command" in
    prepare) [[ $# -eq 2 ]] || fail "usage: $0 prepare true|false"; prepare "$2" ;;
    identity) [[ $# -eq 3 ]] || fail "usage: $0 identity RELEASE_DIR PACKAGED"; identity "$2" "$3" ;;
    metadata) [[ $# -ge 2 ]] || fail "usage: $0 metadata PACKAGED..."; shift; metadata "$@" ;;
    mode) [[ $# -eq 1 ]] || fail "usage: $0 mode"; mode ;;
    prerelease) [[ $# -eq 3 ]] || fail "usage: $0 prerelease REF COUNT"; prerelease "$2" "$3" ;;
    stamp) [[ $# -ge 2 && $# -le 3 ]] || fail "usage: $0 stamp VERSION [ROOT]"; stamp "$2" "${3:-.}" ;;
    *) fail "usage: $0 identity|metadata|mode|prerelease|stamp ..." ;;
esac
