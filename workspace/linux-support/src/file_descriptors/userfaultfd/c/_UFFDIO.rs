// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const _UFFDIO_REGISTER: u64 = 0x00;

pub(super) const _UFFDIO_UNREGISTER: u64 = 0x01;

pub(super) const _UFFDIO_WAKE: u64 = 0x02;

pub(super) const _UFFDIO_COPY: u64 = 0x03;

pub(super) const _UFFDIO_ZEROPAGE: u64 = 0x04;

// Note that value 0x05 has no definition.

pub(super) const _UFFDIO_WRITEPROTECT: u64 = 0x06;

pub(super) const _UFFDIO_API: u64 = 0x3F;
