//! Integration checks shipped in the public archive.

use cargo_plugin_utils::get_owner_repo;

#[test]
fn explicit_repository_identity_needs_both_components() {
    assert!(get_owner_repo(Some("legra-ai".to_owned()), None).is_err());
    assert!(get_owner_repo(None, Some("example".to_owned())).is_err());
    let identity = get_owner_repo(Some("legra-ai".to_owned()), Some("example".to_owned()))
        .expect("complete explicit repository identity");
    assert_eq!(identity, ("legra-ai".to_owned(), "example".to_owned()));
}
