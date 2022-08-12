// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// When set UID 0 has no special privileges.
/// When unset, we support inheritance of root-permissions and suid-root executable under compatibility mode.
/// We raise the effective and inheritable bitmasks *of the executable file* if the effective uid of the new process is 0.
/// If the real uid is 0, we raise the effective (legacy) bit of the executable file.
const SECURE_NOROOT: u32 = 0;

/// Make bit-0 immutable.
const SECURE_NOROOT_LOCKED: u32 = 1;

/// When set, setuid to/from uid 0 does not trigger capability-"fixup".
/// When unset, to provide compatibility with old programs relying on set*uid to gain/lose privilege, transitions to/from uid 0 cause capabilities to be gained/lost.
const SECURE_NO_SETUID_FIXUP: u32 = 2;

/// Make bit-2 immutable.
const SECURE_NO_SETUID_FIXUP_LOCKED: u32 = 3;

/// When set, a process can retain its capabilities even after transitioning to a non-root user (the set-uid fixup suppressed by bit 2).
/// Bit-4 is cleared when a process calls exec(); setting both bit 4 and 5 will create a barrier through exec that `no exec()`'d child can use this feature again.
const SECURE_KEEP_CAPS: u32 = 4;

/// Make bit-4 immutable .
const SECURE_KEEP_CAPS_LOCKED: u32 = 5;

/// When set, a process cannot add new capabilities to its ambient set.
const SECURE_NO_CAP_AMBIENT_RAISE: u32 = 6;

/// Make bit-6 immutable.
const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: u32 = 7;

#[allow(dead_code)] const SECURE_ALL_BITS: u32 = issecure_mask(SECURE_NOROOT) | issecure_mask(SECURE_NO_SETUID_FIXUP) | issecure_mask(SECURE_KEEP_CAPS) | issecure_mask(SECURE_NO_CAP_AMBIENT_RAISE);

#[allow(dead_code)] const SECURE_ALL_LOCKS: u32 = SECURE_ALL_BITS << 1;
