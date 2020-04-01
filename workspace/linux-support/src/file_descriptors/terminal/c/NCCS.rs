// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const NCCS: usize = 32;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const NCCS: usize = 19;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const NCCS: usize = 20;
