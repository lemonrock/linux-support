// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use crate::vectors::VectoredWrite;
use crate::vectors::VectoredRead;


include!("File.AsRawFdExt.rs");include!("File.SpliceRecipient.rs");
include!("File.SpliceSender.rs");
include!("File.SendFile.rs");
include!("File.VectoredRead.rs");
include!("File.VectoredWrite.rs");
include!("SendFile.rs");
