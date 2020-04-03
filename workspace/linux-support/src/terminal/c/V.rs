// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(target_os = "dragonfly")] pub(crate) const VCHECKPT: usize = 19;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VDISCARD: usize = 15;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VDISCARD: usize = 13;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VDISCARD: usize = 16;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const VDSUSP: usize = 11;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VEOF: usize = 0;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VEOF: usize = 4;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VEOL2: usize = 2;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VEOL2: usize = 16;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VEOL2: usize = 8;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VEOL: usize = 1;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VEOL: usize = 11;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VEOL: usize = 6;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))] pub(crate) const VERASE2: usize = 7;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VERASE: usize = 3;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VERASE: usize = 2;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VINTR: usize = 8;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VINTR: usize = 0;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VKILL: usize = 5;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VKILL: usize = 3;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VLNEXT: usize = 14;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VLNEXT: usize = 15;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VMIN: usize = 16;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VMIN: usize = 6;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VMIN: usize = 5;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VQUIT: usize = 9;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VQUIT: usize = 1;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VREPRINT: usize = 6;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VREPRINT: usize = 12;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VREPRINT: usize = 11;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VSTART: usize = 12;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VSTART: usize = 8;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VSTART: usize = 13;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const VSTATUS: usize = 18;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VSTOP: usize = 13;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VSTOP: usize = 9;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VSTOP: usize = 14;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VSUSP: usize = 10;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VSUSP: usize = 10;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VSUSP: usize = 12;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VSWTC: usize = 7;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VSWTC: usize = 9;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VTIME: usize = 17;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VTIME: usize = 5;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VTIME: usize = 7;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const VWERASE: usize = 4;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const VWERASE: usize = 14;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const VWERASE: usize = 10;
