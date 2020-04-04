// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::PathExt;
use crate::paths::PathBufExt;
use crate::paths::SysPath;
use crate::user_and_groups::assert_effective_user_id_is_root;
use libc::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::error;
use std::io;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};
use crate::memory::information::{MemoryInformation, MemoryInformationName};
use crate::memory::page_size;


include!("adjust_transparent_huge_pages.rs");
include!("DisableTransparentHugePagesError.rs");
include!("GlobalHugePagePoolSize.rs");
include!("HugePageSize.rs");
include!("HugePagePoolStatistics.rs");
include!("TransparentHugePageDefragmentationChoice.rs");
include!("TransparentHugePageRegularMemoryChoice.rs");
include!("TransparentHugePagesConfiguration.rs");
include!("TransparentHugePageSharedMemoryChoice.rs");
