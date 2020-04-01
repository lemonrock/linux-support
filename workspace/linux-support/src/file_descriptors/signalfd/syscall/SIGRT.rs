// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// This is the Kernel lower limit of `SIGRTMIN`.
pub const SIGRTMIN_Kernel: c_int = 32;

/// In theory, this is a libc-specific value, but both modern glibc and musl define it as `35` (in the past, when using an older threading implementation, glibc defined it as `34`).
///
/// The kernel lower-limit is 32.
pub const SIGRTMIN: c_int = 35;

/// This value is defined by the kernel.
///
/// There seems to be a bug in musl that defines this value as `127` for MIPS, but the kernel sources disagree.
#[cfg(any(target_arch = "mips64"))] pub const SIGRTMAX: c_int = 128;
/// This value is defined by the kernel.
///
/// There seems to be a bug in musl that defines this value as `127` for MIPS, but the kernel sources disagree.
#[cfg(not(any(target_arch = "mips64")))] pub const SIGRTMAX: c_int = 64;
