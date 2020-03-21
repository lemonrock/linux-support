use libc::*;
use std::error;
use std::io;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

include!("adjust_transparent_huge_pages.rs");
include!("DisableTransparentHugePagesError.rs");
include!("TransparentHugePageDefragmentationChoice.rs");
include!("TransparentHugePageRegularMemoryChoice.rs");
include!("TransparentHugePageSharedMemoryChoice.rs");
