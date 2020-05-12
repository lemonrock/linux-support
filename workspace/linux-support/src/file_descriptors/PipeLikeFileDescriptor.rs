// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A marker trait to bring together all the properties of a pipe-like file descriptor (ie a pipe or character device).
///
/// Use this in conjunction with the marker traits `SpliceSender` and `SpliceRecipient` to distinguish whether a splice-like method supports the use of input or output offets.
///
/// Implementors are not allowed to implement `Seek`.
pub trait PipeLikeFileDescriptor: FileDescriptor
{
}
