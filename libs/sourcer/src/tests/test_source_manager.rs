use crate::{SourceID, SourceManager, SourceText};

// === Constructor Tests ===
#[test]
fn test_source_manager_new() {
    let manager = SourceManager::new();
    // Test that manager is created with no sources
    let non_existent = SourceID::new(999);
    assert!(!manager.has_source(&non_existent));
}

#[test]
fn test_source_manager_new_starts_at_id_10() {
    let mut manager = SourceManager::new();
    let (sid, _) = manager.add_source("first.txt".to_string(), "content1".to_string());
    assert_eq!(sid.id(), 10);
}

// === add_source Tests ===
#[test]
fn test_source_manager_add_source_single() {
    let mut manager = SourceManager::new();
    let (sid, source) = manager.add_source("test.txt".to_string(), "content".to_string());

    assert_eq!(sid.id(), 10);
    assert_eq!(source.path(), "test.txt");
    assert_eq!(source.content(), "content");
    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_add_source_multiple() {
    let mut manager = SourceManager::new();
    let (sid1, _) = manager.add_source("file1.txt".to_string(), "content1".to_string());
    let (sid2, _) = manager.add_source("file2.txt".to_string(), "content2".to_string());
    let (sid3, _) = manager.add_source("file3.txt".to_string(), "content3".to_string());

    assert_eq!(sid1.id(), 10);
    assert_eq!(sid2.id(), 11);
    assert_eq!(sid3.id(), 12);
    assert!(manager.has_source(&sid1));
    assert!(manager.has_source(&sid2));
    assert!(manager.has_source(&sid3));
}

#[test]
fn test_source_manager_add_source_increments_id() {
    let mut manager = SourceManager::new();
    let mut last_id = 10;

    for i in 0..5 {
        let (sid, _) = manager.add_source(format!("file{}.txt", i), format!("content{}", i));
        assert_eq!(sid.id(), last_id);
        last_id += 1;
    }
}

#[test]
fn test_source_manager_add_source_empty_content() {
    let mut manager = SourceManager::new();
    let (sid, source) = manager.add_source("empty.txt".to_string(), String::new());

    assert_eq!(source.content(), "");
    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_add_source_empty_path() {
    let mut manager = SourceManager::new();
    let (sid, source) = manager.add_source(String::new(), "content".to_string());

    assert_eq!(source.path(), "");
    assert_eq!(source.content(), "content");
}

#[test]
fn test_source_manager_add_source_multiline_content() {
    let mut manager = SourceManager::new();
    let content = "line1\nline2\nline3\nline4";
    let (sid, source) = manager.add_source("multiline.txt".to_string(), content.to_string());

    assert_eq!(source.content(), content);
    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_add_source_tamil_content() {
    let mut manager = SourceManager::new();
    let content = "தமிழ்";
    let (sid, source) = manager.add_source("tamil.txt".to_string(), content.to_string());

    assert_eq!(source.content(), "தமிழ்");
    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_add_source_mixed_scripts() {
    let mut manager = SourceManager::new();
    let content = "Hello தமிழ் World 😊";
    let (sid, source) = manager.add_source("mixed.txt".to_string(), content.to_string());

    assert_eq!(source.content(), content);
    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_add_source_long_path() {
    let mut manager = SourceManager::new();
    let long_path = "/very/long/path/to/deeply/nested/file/structure/name/test.txt";
    let (sid, source) = manager.add_source(long_path.to_string(), "content".to_string());

    assert_eq!(source.path(), long_path);
    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_add_source_special_characters() {
    let mut manager = SourceManager::new();
    let path_with_special = "file with spaces & special@chars.txt";
    let (sid, source) = manager.add_source(path_with_special.to_string(), "content".to_string());

    assert_eq!(source.path(), path_with_special);
}

// === get_source Tests ===
#[test]
fn test_source_manager_get_source_exists() {
    let mut manager = SourceManager::new();
    let (sid, _) = manager.add_source("test.txt".to_string(), "content".to_string());

    let retrieved = manager.get_source(&sid);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().path(), "test.txt");
    assert_eq!(retrieved.unwrap().content(), "content");
}

#[test]
fn test_source_manager_get_source_not_exists() {
    let manager = SourceManager::new();
    let non_existent = SourceID::new(999);

    let result = manager.get_source(&non_existent);
    assert!(result.is_none());
}

#[test]
fn test_source_manager_get_source_multiple() {
    let mut manager = SourceManager::new();
    let (sid1, _) = manager.add_source("file1.txt".to_string(), "content1".to_string());
    let (sid2, _) = manager.add_source("file2.txt".to_string(), "content2".to_string());
    let (sid3, _) = manager.add_source("file3.txt".to_string(), "content3".to_string());

    assert_eq!(manager.get_source(&sid1).unwrap().path(), "file1.txt");
    assert_eq!(manager.get_source(&sid2).unwrap().path(), "file2.txt");
    assert_eq!(manager.get_source(&sid3).unwrap().path(), "file3.txt");
}

#[test]
fn test_source_manager_get_source_returns_reference() {
    let mut manager = SourceManager::new();
    let (sid, _) = manager.add_source("test.txt".to_string(), "content".to_string());

    let source1 = manager.get_source(&sid).unwrap();
    let source2 = manager.get_source(&sid).unwrap();

    // Both should have same content
    assert_eq!(source1.path(), source2.path());
    assert_eq!(source1.content(), source2.content());
}

#[test]
fn test_source_manager_get_source_after_multiple_adds() {
    let mut manager = SourceManager::new();
    let (sid_first, _) = manager.add_source("first.txt".to_string(), "first".to_string());

    // Add many more sources
    for i in 0..100 {
        manager.add_source(format!("file{}.txt", i), format!("content{}", i));
    }

    // First source should still be retrievable
    let retrieved = manager.get_source(&sid_first);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().path(), "first.txt");
}

// === has_source Tests ===
#[test]
fn test_source_manager_has_source_true() {
    let mut manager = SourceManager::new();
    let (sid, _) = manager.add_source("test.txt".to_string(), "content".to_string());

    assert!(manager.has_source(&sid));
}

#[test]
fn test_source_manager_has_source_false() {
    let manager = SourceManager::new();
    let non_existent = SourceID::new(999);

    assert!(!manager.has_source(&non_existent));
}

#[test]
fn test_source_manager_has_source_multiple_true() {
    let mut manager = SourceManager::new();
    let (sid1, _) = manager.add_source("file1.txt".to_string(), "content1".to_string());
    let (sid2, _) = manager.add_source("file2.txt".to_string(), "content2".to_string());
    let (sid3, _) = manager.add_source("file3.txt".to_string(), "content3".to_string());

    assert!(manager.has_source(&sid1));
    assert!(manager.has_source(&sid2));
    assert!(manager.has_source(&sid3));
}

#[test]
fn test_source_manager_has_source_multiple_false() {
    let mut manager = SourceManager::new();
    let (sid1, _) = manager.add_source("file1.txt".to_string(), "content1".to_string());
    let (sid2, _) = manager.add_source("file2.txt".to_string(), "content2".to_string());

    assert!(manager.has_source(&sid1));
    assert!(manager.has_source(&sid2));
    assert!(!manager.has_source(&SourceID::new(999)));
}

#[test]
fn test_source_manager_has_source_after_large_adds() {
    let mut manager = SourceManager::new();
    let mut added_ids = Vec::new();

    for i in 0..1000 {
        let (sid, _) = manager.add_source(
            format!("file{}.txt", i),
            format!("content{}", i)
        );
        added_ids.push(sid);
    }

    // Check all added sources exist
    for sid in added_ids {
        assert!(manager.has_source(&sid));
    }

    // Check non-existent ones don't
    assert!(!manager.has_source(&SourceID::new(99999)));
}

#[test]
fn test_source_manager_none_id_not_added() {
    let manager = SourceManager::new();
    assert!(!manager.has_source(&SourceID::NONE));
}

// === Integration Tests ===
#[test]
fn test_source_manager_workflow_single_file() {
    let mut manager = SourceManager::new();

    // Add a file
    let (sid, _) = manager.add_source("main.rs".to_string(), "fn main() {}".to_string());

    // Verify it exists
    assert!(manager.has_source(&sid));

    // Get it
    let source = manager.get_source(&sid).unwrap();
    assert_eq!(source.path(), "main.rs");
    assert_eq!(source.content(), "fn main() {}");

    // Verify different ID doesn't exist
    assert!(!manager.has_source(&SourceID::new(999)));
}

#[test]
fn test_source_manager_workflow_multiple_files() {
    let mut manager = SourceManager::new();

    let (main_sid, _) = manager.add_source("main.rs".to_string(), "fn main() {}".to_string());
    let (lib_sid, _) = manager.add_source("lib.rs".to_string(), "pub fn helper() {}".to_string());
    let (test_sid, _) = manager.add_source("test.rs".to_string(), "#[test] fn test() {}".to_string());

    // Verify all exist
    assert!(manager.has_source(&main_sid));
    assert!(manager.has_source(&lib_sid));
    assert!(manager.has_source(&test_sid));

    // Get specific ones
    assert_eq!(manager.get_source(&main_sid).unwrap().path(), "main.rs");
    assert_eq!(manager.get_source(&lib_sid).unwrap().path(), "lib.rs");
    assert_eq!(manager.get_source(&test_sid).unwrap().path(), "test.rs");
}

#[test]
fn test_source_manager_with_tamil_sources() {
    let mut manager = SourceManager::new();

    let (tamil_sid, _) = manager.add_source(
        "tamil.txt".to_string(),
        "தமிழ் மொழி".to_string()
    );
    let (english_sid, _) = manager.add_source(
        "english.txt".to_string(),
        "English language".to_string()
    );

    assert!(manager.has_source(&tamil_sid));
    assert!(manager.has_source(&english_sid));

    assert_eq!(manager.get_source(&tamil_sid).unwrap().content(), "தமிழ் மொழி");
    assert_eq!(manager.get_source(&english_sid).unwrap().content(), "English language");
}

#[test]
fn test_source_manager_sequential_ids() {
    let mut manager = SourceManager::new();
    let mut last_id = 9;

    for _ in 0..10 {
        let (sid, _) = manager.add_source("test.txt".to_string(), "content".to_string());
        last_id += 1;
        assert_eq!(sid.id(), last_id);
    }
}

#[test]
fn test_source_manager_none_id_never_assigned() {
    let mut manager = SourceManager::new();

    for _ in 0..100 {
        let (sid, _) = manager.add_source("test.txt".to_string(), "content".to_string());
        assert_ne!(sid, SourceID::NONE);
    }
}
