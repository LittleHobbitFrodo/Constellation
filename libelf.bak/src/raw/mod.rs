//! Takes care of the magic happening under the hood

pub(crate) mod types;

pub mod file_header;
pub mod section;
pub mod symbol_table;
pub mod relocations;
pub mod program_headers;
pub mod dynamic;
