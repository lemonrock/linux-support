// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_ACCEPTCONN: c_int = 30;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_ACCEPTCONN: c_int = 0x1009;

#[allow(missing_docs)] pub const SO_ATTACH_BPF: c_int = 50;

#[allow(missing_docs)] pub const SO_ATTACH_FILTER: c_int = 26;

#[allow(missing_docs)] pub const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;

#[allow(missing_docs)] pub const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;

#[allow(missing_docs)] pub const SO_BINDTODEVICE: c_int = 25;

#[allow(missing_docs)] pub const SO_BPF_EXTENSIONS: c_int = 48;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_BROADCAST: c_int = 6;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_BROADCAST: c_int = 0x0020;

#[allow(missing_docs)] pub const SO_BSDCOMPAT: c_int = 14;

#[allow(missing_docs)] pub const SO_BUSY_POLL: c_int = 46;

#[allow(missing_docs)] pub const SO_CNX_ADVICE: c_int = 53;

#[allow(missing_docs)] pub const SO_COOKIE: c_int = 57;

#[allow(missing_docs)] pub const SO_DEBUG: c_int = 1;

#[allow(missing_docs)] pub const SO_DETACH_BPF: c_int = SO_DETACH_FILTER;

#[allow(missing_docs)] pub const SO_DETACH_FILTER: c_int = 27;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_DOMAIN: c_int = 39;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_DOMAIN: c_int = 0x1029;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_DONTROUTE: c_int = 5;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_DONTROUTE: c_int = 0x0010;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_ERROR: c_int = 4;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_ERROR: c_int = 0x1007;

#[allow(missing_docs)] pub const SO_GET_FILTER: c_int = SO_ATTACH_FILTER;

#[allow(missing_docs)] pub const SO_INCOMING_CPU: c_int = 49;

#[allow(missing_docs)] pub const SO_INCOMING_NAPI_ID: c_int = 56;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_KEEPALIVE: c_int = 9;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_KEEPALIVE: c_int = 0x0008;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_LINGER: c_int = 13;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_LINGER: c_int = 0x0080;

#[allow(missing_docs)] pub const SO_LOCK_FILTER: c_int = 44;

#[allow(missing_docs)] pub const SO_MARK: c_int = 36;

#[allow(missing_docs)] pub const SO_MAX_PACING_RATE: c_int = 47;

#[allow(missing_docs)] pub const SO_MEMINFO: c_int = 55;

#[allow(missing_docs)] pub const SO_NOFCS: c_int = 43;

#[allow(missing_docs)] pub const SO_NO_CHECK: c_int = 11;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_OOBINLINE: c_int = 10;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_OOBINLINE: c_int = 0x0100;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_PASSCRED: c_int = 16;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_PASSCRED: c_int = 17;
#[allow(missing_docs)] #[cfg(any(target_arch = "powerpc64"))] pub const SO_PASSCRED: c_int = 20;

#[allow(missing_docs)] pub const SO_PASSSEC: c_int = 34;

#[allow(missing_docs)] pub const SO_PEEK_OFF: c_int = 42;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_PEERCRED: c_int = 17;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_PEERCRED: c_int = 18;
#[allow(missing_docs)] #[cfg(any(target_arch = "powerpc64"))] pub const SO_PEERCRED: c_int = 21;

#[allow(missing_docs)] pub const SO_PEERGROUPS: c_int = 59;

#[allow(missing_docs)] pub const SO_PEERNAME: c_int = 28;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_PEERSEC: c_int = 31;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_PEERSEC: c_int = 30;

#[allow(missing_docs)] pub const SO_PRIORITY: c_int = 12;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_PROTOCOL: c_int = 38;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_PROTOCOL: c_int = 0x1028;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_RCVBUF: c_int = 8;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_RCVBUF: c_int = 0x1002;

#[allow(missing_docs)] pub const SO_RCVBUFFORCE: c_int = 33;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_RCVLOWAT: c_int = 18;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_RCVLOWAT: c_int = 0x1004;
#[allow(missing_docs)] #[cfg(any(target_arch = "powerpc64"))] pub const SO_RCVLOWAT: c_int = 16;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_RCVTIMEO: c_int = 20;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_RCVTIMEO: c_int = 0x1006;
#[allow(missing_docs)] #[cfg(any(target_arch = "powerpc64"))] pub const SO_RCVTIMEO: c_int = 18;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_REUSEADDR: c_int = 2;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_REUSEADDR: c_int = 0x0004;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_REUSEPORT: c_int = 15;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_REUSEPORT: c_int = 0x0200;

#[allow(missing_docs)] pub const SO_RXQ_OVFL: c_int = 40;

#[allow(missing_docs)] pub const SO_SECURITY_AUTHENTICATION: c_int = 22;

#[allow(missing_docs)] pub const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;

#[allow(missing_docs)] pub const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;

#[allow(missing_docs)] pub const SO_SELECT_ERR_QUEUE: c_int = 45;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_SNDBUF: c_int = 7;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_SNDBUF: c_int = 0x1001;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_SNDBUFFORCE: c_int = 32;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_SNDBUFFORCE: c_int = 31;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_SNDLOWAT: c_int = 19;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_SNDLOWAT: c_int = 0x1003;
#[allow(missing_docs)] #[cfg(any(target_arch = "powerpc64"))] pub const SO_SNDLOWAT: c_int = 17;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_SNDTIMEO: c_int = 21;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_SNDTIMEO: c_int = 0x1005;
#[allow(missing_docs)] #[cfg(any(target_arch = "powerpc64"))] pub const SO_SNDTIMEO: c_int = 19;

#[allow(missing_docs)] pub const SO_TIMESTAMP: c_int = 29;

#[allow(missing_docs)] pub const SO_TIMESTAMPING: c_int = 37;

#[allow(missing_docs)] pub const SO_TIMESTAMPNS: c_int = 35;

#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const SO_TYPE: c_int = 3;
#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] pub const SO_TYPE: c_int = 0x1008;

#[allow(missing_docs)] pub const SO_WIFI_STATUS: c_int = 41;

#[allow(missing_docs)] pub const SO_ZEROCOPY: c_int = 60;
