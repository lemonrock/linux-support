// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


pub const CAP_CHOWN: c_uchar = 0;
pub const CAP_DAC_OVERRIDE: c_uchar = 1;
pub const CAP_DAC_READ_SEARCH: c_uchar = 2;
pub const CAP_FOWNER: c_uchar = 3;
pub const CAP_FSETID: c_uchar = 4;
pub const CAP_KILL: c_uchar = 5;
pub const CAP_SETGID: c_uchar = 6;
pub const CAP_SETUID: c_uchar = 7;
pub const CAP_SETPCAP: c_uchar = 8;
pub const CAP_LINUX_IMMUTABLE: c_uchar = 9;
pub const CAP_NET_BIND_SERVICE: c_uchar = 10;
pub const CAP_NET_BROADCAST: c_uchar = 11;
pub const CAP_NET_ADMIN: c_uchar = 12;
pub const CAP_NET_RAW: c_uchar = 13;
pub const CAP_IPC_LOCK: c_uchar = 14;
pub const CAP_IPC_OWNER: c_uchar = 15;
pub const CAP_SYS_MODULE: c_uchar = 16;
pub const CAP_SYS_RAWIO: c_uchar = 17;
pub const CAP_SYS_CHROOT: c_uchar = 18;
pub const CAP_SYS_PTRACE: c_uchar = 19;
pub const CAP_SYS_PACCT: c_uchar = 20;
pub const CAP_SYS_ADMIN: c_uchar = 21;
pub const CAP_SYS_BOOT: c_uchar = 22;
pub const CAP_SYS_NICE: c_uchar = 23;
pub const CAP_SYS_RESOURCE: c_uchar = 24;
pub const CAP_SYS_TIME: c_uchar = 25;
pub const CAP_SYS_TTY_CONFIG: c_uchar = 26;
pub const CAP_MKNOD: c_uchar = 27;
pub const CAP_LEASE: c_uchar = 28;
pub const CAP_AUDIT_WRITE: c_uchar = 29;
pub const CAP_AUDIT_CONTROL: c_uchar = 30;
pub const CAP_SETFCAP: c_uchar = 31;

// All of these capabilities can not be represented in the capabilities set API
pub const CAP_MAC_OVERRIDE: c_uchar = 32;
pub const CAP_MAC_ADMIN: c_uchar = 33;
pub const CAP_SYSLOG: c_uchar = 34;
pub const CAP_WAKE_ALARM: c_uchar = 35;
pub const CAP_BLOCK_SUSPEND: c_uchar = 36;
pub const CAP_AUDIT_READ: c_uchar = 37;

pub const CAP_LAST_CAP: size_t = 37;
