// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const UFFDIO_API: u64 = _IOWR::<uffdio_api>(UFFDIO, _UFFDIO_API);

pub(super) const UFFDIO_REGISTER: u64 = _IOWR::<uffdio_register>(UFFDIO, _UFFDIO_REGISTER);

pub(super) const UFFDIO_UNREGISTER: u64 = _IOR::<uffdio_range>(UFFDIO, _UFFDIO_UNREGISTER);

pub(super) const UFFDIO_WAKE: u64 = _IOR::<uffdio_range>(UFFDIO, _UFFDIO_WAKE);

pub(super) const UFFDIO_COPY: u64 = _IOWR::<uffdio_copy>(UFFDIO, _UFFDIO_WAKE);

pub(super) const UFFDIO_ZEROPAGE: u64 = _IOWR::<uffdio_zeropage>(UFFDIO, _UFFDIO_ZEROPAGE);

pub(super) const UFFDIO_WRITEPROTECT: u64 = _IOWR::<uffdio_writeprotect>(UFFDIO, _UFFDIO_WRITEPROTECT);
