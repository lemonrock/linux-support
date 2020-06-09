// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::c::*;
use crate::memory::NonZeroNumberOfPages;
use crate::memory::PageSize;
use crate::paths::ProcPath;
use crate::paths::PathExt;
use crate::user_and_groups::assert_effective_user_id_is_root;


pub(crate) mod c;


include!("ChangeCapacityError.rs");
include!("maximum_pipe_capacity.rs");
include!("pipe_user_pages_hard_limit.rs");
include!("pipe_user_pages_soft_limit.rs");
include!("PipeFileDescriptor.rs");
include!("ReceivePipeFileDescriptor.rs");
include!("SendPipeFileDescriptor.rs");
include!("set_maximum_pipe_capacity.rs");
include!("set_pipe_user_pages_hard_limit.rs");
include!("set_pipe_user_pages_soft_limit.rs");
include!("SpliceFlags.rs");
include!("SpliceRecipient.rs");
include!("SpliceSender.rs");
