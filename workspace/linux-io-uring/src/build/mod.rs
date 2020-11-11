// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::Path;


pub(super) mod naptr_service_parser;


pub(super) mod rerun_if_changed;


pub(super) mod top_level_domains;


include!("new_buf_writer.rs");
