// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use self::c::*;
use super::*;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use crate::memory::PageSize;
use crate::paths::PathExt;
use crate::paths::ProcPath;
use crate::process::ProcessIdentifierChoice;
use crate::process::ProcessIdentifier;
use crate::inode::*;
use crate::inode::c::*;


/// C data structures.
pub mod c;


/// Leasing.
pub mod leasing;


/// Locking.
pub mod locking;


include!("Advice.rs");
include!("Advise.rs");
include!("Allocate.rs");
include!("AllocationMode.rs");
include!("CopyFileRange.rs");
include!("CopyOnWrite.rs");
include!("Deduplicate.rs");
include!("DeduplicationOutcome.rs");
include!("ExtendedSeek.rs");
include!("ExtendedSeekFrom.rs");
include!("File.Allocate.rs");
include!("File.CopyFileRange.rs");
include!("File.CopyOnWrite.rs");
include!("File.FileDescriptor.rs");
include!("File.MemoryMappableFileDescriptor.rs");
include!("File.OnDiskFileDescriptor.rs");
include!("File.SeekableFileDescriptor.rs");
include!("File.SpliceRecipient.rs");
include!("File.SpliceSender.rs");
include!("File.SendFile.rs");
include!("File.Sparseness.rs");
include!("File.Synchronize.rs");
include!("File.VectoredRead.rs");
include!("File.VectoredWrite.rs");
include!("ReadVectoredFlags.rs");
include!("ReadAhead.rs");
include!("ReadOrWriteOffset.rs");
include!("SendFile.rs");
include!("Sparseness.rs");
include!("Synchronize.rs");
include!("SynchronizeFileRangeFlags.rs");
include!("WriteVectoredFlags.rs");
