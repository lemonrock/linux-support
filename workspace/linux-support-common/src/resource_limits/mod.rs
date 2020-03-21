use super::paths::ProcPath;
use errno::errno;
use libc::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io;


include!("ResourceName.rs");
include!("ResourceLimit.rs");
include!("ResourceLimitsSet.rs");
include!("SoftAndHardResourceLimit.rs");
