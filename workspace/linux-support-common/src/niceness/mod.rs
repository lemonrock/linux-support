use super::paths::ProcPath;
use errno::errno;
use libc::*;
use likely::likely;
use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;


include!("Nice.rs");
include!("ProcessNiceness.rs");
include!("ProcessNicenessAdjustmentError.rs");
