//! The sourcer library provides a unified interface for managing source code from various origins.
//!
//! It abstracts different source types (files, stdin, virtual) into a common structure,
//! allowing downstream processes to work with SourceID and SourceText without worrying about
//! the underlying source details.

#![allow(unused)]

mod loaders;
mod source_id;
mod source_manager;
mod source_text;
mod tests;
mod span;
mod errors;

pub use loaders::{load_from_file, load_from_stdin, load_virtual};
pub use source_id::SourceID;
pub use source_manager::SourceManager;
pub use source_text::SourceText;
pub use span::Span;
pub use errors::{SourcerError, SourcerResult};
