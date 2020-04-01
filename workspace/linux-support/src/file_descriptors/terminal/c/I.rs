// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IGNBRK: tcflag_t = 0x00000001;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IGNBRK: tcflag_t = 0o000001;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const BRKINT: tcflag_t = 0x00000002;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const BRKINT: tcflag_t = 0o000002;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IGNPAR: tcflag_t = 0x00000004;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IGNPAR: tcflag_t = 0o000004;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const PARMRK: tcflag_t = 0x00000008;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const PARMRK: tcflag_t = 0o000010;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const INPCK: tcflag_t = 0x00000010;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const INPCK: tcflag_t = 0o000020;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ISTRIP: tcflag_t = 0x00000020;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const ISTRIP: tcflag_t = 0o000040;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const INLCR: tcflag_t = 0x00000040;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const INLCR: tcflag_t = 0o000100;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IGNCR: tcflag_t = 0x00000080;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IGNCR: tcflag_t = 0o000200;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ICRNL: tcflag_t = 0x00000100;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const ICRNL: tcflag_t = 0o000400;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IUCLC: tcflag_t = 0o001000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IXON: tcflag_t = 0x00000200;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IXON: tcflag_t = 0o002000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IXOFF: tcflag_t = 0x00000400;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IXOFF: tcflag_t = 0o010000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IXANY: tcflag_t = 0x00000800;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IXANY: tcflag_t = 0o004000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IMAXBEL: tcflag_t = 0x00002000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IMAXBEL: tcflag_t = 0o020000;

#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const IUTF8: tcflag_t = 0x00004000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const IUTF8: tcflag_t = 0o040000;



