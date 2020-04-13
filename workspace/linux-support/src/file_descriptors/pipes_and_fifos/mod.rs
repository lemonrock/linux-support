// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use crate::vectors::VectoredRead;
use crate::vectors::VectoredWrite;
use crate::paths::{ProcPath, PathExt};
use crate::user_and_groups::assert_effective_user_id_is_root;


pub(crate) mod syscall;


include!("ChangeCapacityError.rs");
include!("PipeFileDescriptor.rs");
include!("ReceivePipeFileDescriptor.rs");
include!("SendPipeFileDescriptor.rs");
include!("SpliceRecipient.rs");
include!("SpliceSender.rs");
