use libc::*;
use likely::unlikely;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;


include!("c_string_pointer_to_path_buf.rs");
include!("ConstCStr.rs");
include!("OsStrExtMore.rs");
include!("path_to_cstring.rs");
include!("replace.rs");
include!("split.rs");
include!("splitn.rs");
include!("to_c_string_robustly.rs");
