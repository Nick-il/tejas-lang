use crate::{load_virtual, SourceManager};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

// === load_virtual Tests ===
#[test]
fn test_load_virtual_simple() {
    let mut manager = SourceManager::new();
    let (sid, source) = load_virtual(&mut manager, "virtual.txt", "virtual content");

    assert_eq!(source.path(), "virtual.txt");
    assert_eq!(source.content(), "virtual content");
    assert!(manager.has_source(&sid));
}

#[test]
fn test_load_virtual_empty_content() {
    let mut manager = SourceManager::new();
    let (sid, source) = load_virtual(&mut manager, "empty.txt", "");

    assert_eq!(source.path(), "empty.txt");
    assert_eq!(source.content(), "");
    assert!(manager.has_source(&sid));
}

#[test]
fn test_load_virtual_empty_path() {
    let mut manager = SourceManager::new();
    let (sid, source) = load_virtual(&mut manager, "", "content");

    assert_eq!(source.path(), "");
    assert_eq!(source.content(), "content");
}

#[test]
fn test_load_virtual_multiline() {
    let mut manager = SourceManager::new();
    let content = "line1\nline2\nline3";
    let (sid, source) = load_virtual(&mut manager, "multiline.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_tamil_content() {
    let mut manager = SourceManager::new();
    let (sid, source) = load_virtual(&mut manager, "tamil.txt", "தமிழ் மொழி");

    assert_eq!(source.path(), "tamil.txt");
    assert_eq!(source.content(), "தமிழ் மொழி");
}

#[test]
fn test_load_virtual_mixed_scripts() {
    let mut manager = SourceManager::new();
    let content = "Hello தமிழ் World 😊";
    let (sid, source) = load_virtual(&mut manager, "mixed.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_emoji() {
    let mut manager = SourceManager::new();
    let content = "Hello 👋 World 😊 🎉";
    let (sid, source) = load_virtual(&mut manager, "emoji.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_special_characters() {
    let mut manager = SourceManager::new();
    let content = "Special: !@#$%^&*()_+-=[]{}|;:',.<>?/\\~`";
    let (sid, source) = load_virtual(&mut manager, "special.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_tabs_and_spaces() {
    let mut manager = SourceManager::new();
    let content = "\t\tindented\n  spaced\n\t mixed";
    let (sid, source) = load_virtual(&mut manager, "whitespace.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_long_content() {
    let mut manager = SourceManager::new();
    let content = "x".repeat(100_000);
    let (sid, source) = load_virtual(&mut manager, "long.txt", &content);

    assert_eq!(source.content().len(), 100_000);
}

#[test]
fn test_load_virtual_multiple() {
    let mut manager = SourceManager::new();

    let (sid1, _) = load_virtual(&mut manager, "virtual1.txt", "content1");
    let (sid2, _) = load_virtual(&mut manager, "virtual2.txt", "content2");
    let (sid3, _) = load_virtual(&mut manager, "virtual3.txt", "content3");

    assert!(manager.has_source(&sid1));
    assert!(manager.has_source(&sid2));
    assert!(manager.has_source(&sid3));

    assert_eq!(manager.get_source(&sid1).unwrap().content(), "content1");
    assert_eq!(manager.get_source(&sid2).unwrap().content(), "content2");
    assert_eq!(manager.get_source(&sid3).unwrap().content(), "content3");
}

#[test]
fn test_load_virtual_consecutive_loads() {
    let mut manager = SourceManager::new();
    let mut ids = Vec::new();

    for i in 0..10 {
        let (sid, _) = load_virtual(&mut manager, &format!("virtual{}.txt", i), &format!("content{}", i));
        ids.push(sid);
    }

    // Verify all are stored
    for (i, sid) in ids.iter().enumerate() {
        assert!(manager.has_source(sid));
        assert_eq!(manager.get_source(sid).unwrap().content(), format!("content{}", i));
    }
}

#[test]
fn test_load_virtual_newline_variations() {
    let mut manager = SourceManager::new();

    // LF only
    let (sid_lf, _) = load_virtual(&mut manager, "lf.txt", "line1\nline2");
    assert_eq!(manager.get_source(&sid_lf).unwrap().content(), "line1\nline2");
}

#[test]
fn test_load_virtual_unicode_combining() {
    let mut manager = SourceManager::new();
    // Combining characters
    let content = "e\u{0301}"; // é as e + combining acute
    let (sid, source) = load_virtual(&mut manager, "combining.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_zero_width_characters() {
    let mut manager = SourceManager::new();
    // Zero-width joiner
    let content = "👨‍👩‍👧‍👦"; // Family emoji (multiple characters with ZWJ)
    let (sid, source) = load_virtual(&mut manager, "zwj.txt", content);

    assert_eq!(source.content(), content);
}

#[test]
fn test_load_virtual_path_with_path_separators() {
    let mut manager = SourceManager::new();
    let (sid, source) = load_virtual(&mut manager, "path/to/file/name.txt", "content");

    assert_eq!(source.path(), "path/to/file/name.txt");
    assert_eq!(source.content(), "content");
}

#[test]
fn test_load_virtual_path_with_special_chars() {
    let mut manager = SourceManager::new();
    let path = "file@#$%^&*().rs";
    let (sid, source) = load_virtual(&mut manager, path, "content");

    assert_eq!(source.path(), path);
}

// === load_from_file Tests ===
#[test]
fn test_load_from_file_simple() {
    let mut manager = SourceManager::new();

    // Create a temporary file
    let temp_file = "/tmp/test_sourcer_simple.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all(b"test content").expect("Failed to write to test file");
    }

    // Load it
    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (sid, source) = result.unwrap();
    assert_eq!(source.content(), "test content");
    assert!(manager.has_source(&sid));

    // Cleanup
    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_multiline() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_multiline.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all(b"line1\nline2\nline3").expect("Failed to write to test file");
    }

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (_, source) = result.unwrap();
    assert_eq!(source.content(), "line1\nline2\nline3");

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_empty() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_empty.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        // Don't write anything
    }

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (_, source) = result.unwrap();
    assert_eq!(source.content(), "");

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_not_found() {
    let mut manager = SourceManager::new();

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new("/tmp/nonexistent_file_12345.txt"));
    assert!(result.is_err());
}

#[test]
fn test_load_from_file_tamil() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_tamil.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all("தமிழ் மொழி".as_bytes()).expect("Failed to write to test file");
    }

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (_, source) = result.unwrap();
    assert_eq!(source.content(), "தமிழ் மொழி");

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_emoji() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_emoji.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all("Hello 👋 World 😊".as_bytes()).expect("Failed to write to test file");
    }

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (_, source) = result.unwrap();
    assert_eq!(source.content(), "Hello 👋 World 😊");

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_path_preservation() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_path.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all(b"content").expect("Failed to write to test file");
    }

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (_, source) = result.unwrap();
    assert_eq!(source.path(), temp_file);

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_large_file() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_large.txt";
    let large_content = "x".repeat(1_000_000);
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all(large_content.as_bytes()).expect("Failed to write to test file");
    }

    let result = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file));
    assert!(result.is_ok());

    let (_, source) = result.unwrap();
    assert_eq!(source.content().len(), 1_000_000);

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_load_from_file_multiple_files() {
    let temp_file1 = "/tmp/test_sourcer_multi1.txt";
    let temp_file2 = "/tmp/test_sourcer_multi2.txt";

    {
        let mut file = fs::File::create(temp_file1).expect("Failed to create test file 1");
        file.write_all(b"content1").expect("Failed to write to test file 1");
    }
    {
        let mut file = fs::File::create(temp_file2).expect("Failed to create test file 2");
        file.write_all(b"content2").expect("Failed to write to test file 2");
    }

    let mut manager = SourceManager::new();
    let result1 = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file1));
    assert!(result1.is_ok());
    let (sid1, source1) = result1.unwrap();
    assert_eq!(source1.content(), "content1");

    let result2 = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file2));
    assert!(result2.is_ok());
    let (sid2, source2) = result2.unwrap();
    assert_eq!(source2.content(), "content2");

    assert_ne!(sid1, sid2);

    let _ = fs::remove_file(temp_file1);
    let _ = fs::remove_file(temp_file2);
}

// === Integration Tests ===
#[test]
fn test_workflow_mixed_virtual_and_file() {
    let mut manager = SourceManager::new();

    // Add virtual source
    let (v_sid, _) = load_virtual(&mut manager, "virtual.rs", "fn virtual_main() {}");

    // Add file source
    let temp_file = "/tmp/test_sourcer_integration.rs";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all(b"fn main() {}").expect("Failed to write to test file");
    }

    let (f_sid, _) = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file)).unwrap();

    // Both should be accessible
    assert!(manager.has_source(&v_sid));
    assert!(manager.has_source(&f_sid));
    assert_ne!(v_sid, f_sid);

    let _ = fs::remove_file(temp_file);
}

#[test]
fn test_workflow_tamil_and_english_sources() {
    let mut manager = SourceManager::new();

    let (tamil_sid, _) = load_virtual(&mut manager, "tamil.txt", "தமிழ் மொழி");
    let (english_sid, _) = load_virtual(&mut manager, "english.txt", "English language");

    assert_eq!(manager.get_source(&tamil_sid).unwrap().content(), "தமிழ் மொழி");
    assert_eq!(manager.get_source(&english_sid).unwrap().content(), "English language");
}

#[test]
fn test_workflow_mixed_language_files() {
    let mut manager = SourceManager::new();

    let temp_file = "/tmp/test_sourcer_mixed_lang.txt";
    {
        let mut file = fs::File::create(temp_file).expect("Failed to create test file");
        file.write_all("English தமிழ் mixed 😊".as_bytes()).expect("Failed to write to test file");
    }

    let (sid, source) = crate::loaders::load_from_file(&mut manager, std::path::Path::new(temp_file)).unwrap();

    assert_eq!(source.content(), "English தமிழ் mixed 😊");

    let _ = fs::remove_file(temp_file);
}
