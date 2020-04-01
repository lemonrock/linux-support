// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B0: speed_t = 0o000000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B0: speed_t = 0;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B50: speed_t = 0o000001;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B50: speed_t = 50;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B75: speed_t = 0o000002;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B75: speed_t = 75;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B110: speed_t = 0o000003;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B110: speed_t = 110;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B134: speed_t = 0o000004;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B134: speed_t = 134;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B150: speed_t = 0o000005;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B150: speed_t = 150;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B200: speed_t = 0o000006;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B200: speed_t = 200;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B300: speed_t = 0o000007;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B300: speed_t = 300;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B600: speed_t = 0o000010;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B600: speed_t = 600;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B1200: speed_t = 0o000011;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B1200: speed_t = 1200;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B1800: speed_t = 0o000012;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B1800: speed_t = 1800;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B2400: speed_t = 0o000013;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B2400: speed_t = 2400;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B4800: speed_t = 0o000014;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B4800: speed_t = 4800;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B9600: speed_t = 0o000015;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B9600: speed_t = 9600;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B19200: speed_t = 0o000016;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B19200: speed_t = 19200;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const B38400: speed_t = 0o000017;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B38400: speed_t = 38400;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B57600: speed_t = 0o010001;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B57600: speed_t = 0o020;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B57600: speed_t = 57600;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B115200: speed_t = 0o010002;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B115200: speed_t = 0o021;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B115200: speed_t = 115200;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B230400: speed_t = 0o010003;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B230400: speed_t = 0o022;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const B230400: speed_t = 230400;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B460800: speed_t = 0o010004;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B460800: speed_t = 0o023;
#[cfg(any(target_os = "freebsd"))] pub(crate) const B460800: speed_t = 460800;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B500000: speed_t = 0o010005;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B500000: speed_t = 0o024;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B576000: speed_t = 0o010006;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B576000: speed_t = 0o025;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B921600: speed_t = 0o010007;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B921600: speed_t = 0o026;
#[cfg(any(target_os = "freebsd"))] pub(crate) const B921600: speed_t = 921600;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B1000000: speed_t = 0o010010;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B1000000: speed_t = 0o027;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B1152000: speed_t = 0o010011;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B1152000: speed_t = 0o030;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B1500000: speed_t = 0o010012;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B1500000: speed_t = 0o031;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B2000000: speed_t = 0o010013;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B2000000: speed_t = 0o032;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B2500000: speed_t = 0o010014;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B2500000: speed_t = 0o033;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B3000000: speed_t = 0o010015;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B3000000: speed_t = 0o034;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B3500000: speed_t = 0o010016;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B3500000: speed_t = 0o035;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc64")))] pub(crate) const B4000000: speed_t = 0o010017;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc64"))] pub(crate) const B4000000: speed_t = 0o036;
