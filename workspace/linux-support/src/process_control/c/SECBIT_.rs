// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) const SECBIT_NOROOT: u32 = issecure_mask(SECURE_NOROOT);

pub(crate) const SECBIT_NOROOT_LOCKED: u32 = issecure_mask(SECURE_NOROOT_LOCKED);

pub(crate) const SECBIT_NO_SETUID_FIXUP: u32 = issecure_mask(SECURE_NO_SETUID_FIXUP);

pub(crate) const SECBIT_NO_SETUID_FIXUP_LOCKED: u32 = issecure_mask(SECURE_NO_SETUID_FIXUP_LOCKED);

pub(crate) const SECBIT_KEEP_CAPS: u32 = issecure_mask(SECURE_KEEP_CAPS);

pub(crate) const SECBIT_KEEP_CAPS_LOCKED: u32 = issecure_mask(SECURE_KEEP_CAPS_LOCKED);

pub(crate) const SECBIT_NO_CAP_AMBIENT_RAISE: u32 = issecure_mask(SECURE_NO_CAP_AMBIENT_RAISE);

pub(crate) const SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED: u32 = issecure_mask(SECURE_NO_CAP_AMBIENT_RAISE_LOCKED);
