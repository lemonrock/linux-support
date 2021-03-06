// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const UFFD_EVENT_PAGEFAULT: u8 = 0x12;

pub(super) const UFFD_EVENT_FORK: u8 = 0x13;

pub(super) const UFFD_EVENT_REMAP: u8 = 0x14;

pub(super) const UFFD_EVENT_REMOVE: u8 = 0x15;

pub(super) const UFFD_EVENT_UNMAP: u8 = 0x16;
