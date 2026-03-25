use crate::SourceID;

#[test]
fn test_source_id_new() {
    let id = SourceID::new(42);
    assert_eq!(id.id(), 42);
}

#[test]
fn test_source_id_new_zero() {
    let id = SourceID::new(0);
    assert_eq!(id.id(), 0);
}

#[test]
fn test_source_id_new_max() {
    let id = SourceID::new(u32::MAX);
    assert_eq!(id.id(), u32::MAX);
}

#[test]
fn test_source_id_none() {
    assert_eq!(SourceID::NONE.id(), 0);
}

#[test]
fn test_source_id_display() {
    let id = SourceID::new(123);
    assert_eq!(format!("{}", id), "SourceID(123)");
}

#[test]
fn test_source_id_display_zero() {
    let id = SourceID::new(0);
    assert_eq!(format!("{}", id), "SourceID(0)");
}

#[test]
fn test_source_id_debug() {
    let id = SourceID::new(456);
    assert_eq!(format!("{:?}", id), "SourceID(456)");
}

#[test]
fn test_source_id_debug_equals_display() {
    let id = SourceID::new(789);
    assert_eq!(format!("{:?}", id), format!("{}", id));
}

#[test]
fn test_source_id_equality() {
    let id1 = SourceID::new(10);
    let id2 = SourceID::new(10);
    let id3 = SourceID::new(20);
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_source_id_equality_none() {
    assert_eq!(SourceID::NONE, SourceID::new(0));
    assert_ne!(SourceID::NONE, SourceID::new(1));
}

#[test]
fn test_source_id_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let id1 = SourceID::new(1);
    let id2 = SourceID::new(1);
    let id3 = SourceID::new(2);

    set.insert(id1);
    set.insert(id2); // Should not add duplicate
    set.insert(id3);

    assert_eq!(set.len(), 2);
}

#[test]
fn test_source_id_copy() {
    let id1 = SourceID::new(5);
    let id2 = id1; // Copy semantics
    assert_eq!(id1, id2);
    assert_eq!(id1.id(), 5);
}
