//! Integration tests for specification loading.

use std::fs;
use std::path::{Path, PathBuf};

use vp_reference_spec::{SpecificationLoadOptions, SpecificationLoader};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn install_registry_fixtures(root: &Path) {
    let rfc_source =
        fixtures_dir().join("../../../veritypay-tooling/crates/vp-registry/tests/fixtures/valid");
    let term_source = fixtures_dir()
        .join("../../../veritypay-tooling/crates/vp-registry/tests/fixtures/term/valid");

    fs::create_dir_all(root.join("spec/rfcs")).expect("spec/rfcs");
    fs::create_dir_all(root.join("spec/terminology")).expect("spec/terminology");
    fs::copy(
        rfc_source.join("registry.yaml"),
        root.join("spec/rfcs/registry.yaml"),
    )
    .expect("copy rfc registry");
    fs::copy(
        term_source.join("registry.yaml"),
        root.join("spec/terminology/registry.yaml"),
    )
    .expect("copy term registry");
}

fn install_minimal_spec_tree(root: &Path) {
    install_registry_fixtures(root);
    fs::create_dir_all(root.join("docs/overview")).expect("docs");
    fs::write(
        root.join("docs/overview/page.md"),
        "# Overview\n\nSee [VP-RFC-0000](../rfcs/0000-rfc-process.md).\n",
    )
    .expect("write doc");
}

#[test]
fn loads_fixture_spec_tree_through_spec_model() {
    let dir = tempfile::tempdir().expect("tempdir");
    install_minimal_spec_tree(dir.path());

    let loaded = SpecificationLoader::new()
        .load(&SpecificationLoadOptions::new(dir.path()))
        .expect("load");

    let context = loaded.context();
    assert!(context.summary.term_count > 0);
    assert!(context.summary.rfc_count > 0);
    assert!(context.summary.document_count > 0);
    assert!(!context.spec_root_identity.is_empty());
}

#[test]
fn missing_spec_root_fails_clearly() {
    let missing = PathBuf::from("/nonexistent/veritypay-spec-root-for-test");

    let error = SpecificationLoader::new()
        .load(&SpecificationLoadOptions::new(missing))
        .expect_err("missing root");

    assert!(error.to_string().contains("does not exist"));
}

#[test]
fn specification_context_contains_counts() {
    let dir = tempfile::tempdir().expect("tempdir");
    install_minimal_spec_tree(dir.path());

    let loaded = SpecificationLoader::new()
        .load(&SpecificationLoadOptions::new(dir.path()))
        .expect("load");

    let summary = &loaded.context().summary;
    assert_eq!(
        summary.term_count,
        loaded
            .specification()
            .registry_set
            .terminology
            .entries()
            .len()
    );
    assert_eq!(
        summary.rfc_count,
        loaded.specification().registry_set.rfcs.entries().len()
    );
    assert_eq!(
        summary.document_count,
        loaded.specification().document_corpus.documents().len()
    );
    assert_eq!(
        summary.reference_edge_count,
        loaded.specification().reference_graph().edges().len()
    );
}

#[test]
fn interpreter_crate_does_not_depend_on_vp_reference_spec() {
    let manifest =
        fs::read_to_string(fixtures_dir().join("../vp-reference-interpreter/Cargo.toml"))
            .expect("read interpreter manifest");
    assert!(
        !manifest.contains("vp-reference-spec"),
        "interpreter must not depend on vp-reference-spec per ADR-0002"
    );
}

#[test]
fn optional_sibling_veritypay_spec_loads_when_present() {
    let sibling = fixtures_dir().join("../../../veritypay-spec");
    if !sibling.is_dir() {
        eprintln!("skipping: sibling ../veritypay-spec not found");
        return;
    }

    let loaded = SpecificationLoader::new()
        .load(&SpecificationLoadOptions::new(&sibling))
        .expect("load sibling spec");

    assert!(loaded.context().summary.term_count > 0);
    assert!(loaded.context().summary.rfc_count > 0);
    assert!(loaded.context().summary.document_count > 0);
}
