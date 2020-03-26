// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::cpu::HyperThread;
use crate::memory::huge_pages::*;
use crate::memory::information::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use crate::paths::*;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;
use std::num::TryFromIntError;


include!("NumaNode.rs");
include!("NumaNodeBitmask.rs");
