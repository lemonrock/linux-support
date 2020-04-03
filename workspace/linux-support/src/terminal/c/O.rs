// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const OPOST: tcflag_t = 0o000001;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const OPOST: tcflag_t = 0x00000001;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const OLCUC: tcflag_t = 0o000002;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const ONLCR: tcflag_t = 0o000004;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ONLCR: tcflag_t = 0x00000002;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const OCRNL: tcflag_t = 0o000010;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const OCRNL: tcflag_t = 0x00000010;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const ONOCR: tcflag_t = 0o000020;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const ONOCR: tcflag_t = 0o000020;
#[cfg(target_os = "openbsd")] pub(crate) const ONOCR: tcflag_t = 0x00000040;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const ONLRET: tcflag_t = 0o000040;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const ONLRET: tcflag_t = 0o000020;
#[cfg(target_os = "openbsd")] pub(crate) const ONLRET: tcflag_t = 0x00000080;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const OFILL: tcflag_t = 0o000100;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const OFILL: tcflag_t = 0x00000080;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const OFDEL: tcflag_t = 0o000200;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const OFDEL: tcflag_t = 0x00020000;

/// New line (`NL`, also known as line feed) delay bitmask.
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const NLDLY: tcflag_t = 0o000400;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const NLDLY: tcflag_t = 0x00000300;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const NL0: tcflag_t = 0o000000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const NL0: tcflag_t = 0x00000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const NL1: tcflag_t = 0o000400;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const NL1: tcflag_t = 0x00000100;

#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const NL2: tcflag_t = 0x00000200;

#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const NL3: tcflag_t = 0x00000300;

/// Carriage return (`CR`) delay bitmask.
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CRDLY: tcflag_t = 0o003000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const CRDLY: tcflag_t = 0x00003000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CR0: tcflag_t = 0o000000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const CR0: tcflag_t = 0x00000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CR1: tcflag_t = 0o001000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const CR1: tcflag_t = 0x00001000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CR2: tcflag_t = 0o002000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const CR2: tcflag_t = 0x00002000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CR3: tcflag_t = 0o003000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const CR3: tcflag_t = 0x00003000;

/// Horizontal tab (`HT` or `TAB`) delay bitmask.
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const TABDLY: tcflag_t = 0o014000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const TABDLY: tcflag_t = 0x00000c04;
#[cfg(target_os = "freebsd")] pub(crate) const TABDLY: tcflag_t = 0x00000004;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const TAB0: tcflag_t = 0o000000;
#[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const TAB0: tcflag_t = 0x00000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const TAB1: tcflag_t = 0o004000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const TAB1: tcflag_t = 0x00000400;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const TAB2: tcflag_t = 0o010000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const TAB2: tcflag_t = 0x00000800;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const TAB3: tcflag_t = 0o014000;
#[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const TAB3: tcflag_t = 0x00000004;

/// Backspace (`BS`) delay bitmask.
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const BSDLY: tcflag_t = 0o020000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const BSDLY: tcflag_t = 0x00008000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const BS0: tcflag_t = 0o000000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const BS0: tcflag_t = 0x00000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const BS1: tcflag_t = 0o020000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const BS1: tcflag_t = 0x00008000;

/// Form feed (`FF`( delay bitmask.
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const FFDLY: tcflag_t = 0o100000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const FFDLY: tcflag_t = 0x00004000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const FF0: tcflag_t = 0o000000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const FF0: tcflag_t = 0x00000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const FF1: tcflag_t = 0o100000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const FF1: tcflag_t = 0x00004000;

/// Vertical tab (`VT`) delay bitmask.
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VTDLY: tcflag_t = 0o040000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const VTDLY: tcflag_t = 0x00010000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VT0: tcflag_t = 0o000000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const VT0: tcflag_t = 0x00000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const VT1: tcflag_t = 0o040000;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) const VT1: tcflag_t = 0x00010000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const XTABS: tcflag_t = 0o014000;

#[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const OXTABS: tcflag_t = TAB3;
#[cfg(target_os = "dragonfly")] pub(crate) const OXTABS: tcflag_t = 0x00000004;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ONOEOT: tcflag_t = 0x00000008;
