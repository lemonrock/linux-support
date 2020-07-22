// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] pub(crate) const SOL_SOCKET: c_int = 0xFFFF;
#[cfg(not(any(target_arch = "mips64", target_arch = "sparc64")))] #[allow(missing_docs)] pub(crate) const SOL_SOCKET: c_int = 1;

#[allow(missing_docs)] pub const SOL_TCP: c_int = 6;

#[allow(missing_docs)] pub const SOL_UDP: c_int = 17;

#[allow(missing_docs)] pub const SOL_XDP: c_int = 283;
