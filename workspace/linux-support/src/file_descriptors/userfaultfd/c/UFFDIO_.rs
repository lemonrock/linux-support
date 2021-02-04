// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const UFFDIO_API: i32 = _IOWR::<uffdio_api>(UFFDIO as u32, _UFFDIO_API as u32);

pub(super) const UFFDIO_REGISTER: i32 = _IOWR::<uffdio_register>(UFFDIO as u32, _UFFDIO_REGISTER as u32);

pub(super) const UFFDIO_UNREGISTER: i32 = _IOR::<uffdio_range>(UFFDIO as u32, _UFFDIO_UNREGISTER as u32);

pub(super) const UFFDIO_WAKE: i32 = _IOR::<uffdio_range>(UFFDIO as u32, _UFFDIO_WAKE as u32);

pub(super) const UFFDIO_COPY: i32 = _IOWR::<uffdio_copy>(UFFDIO as u32, _UFFDIO_COPY as u32);

pub(super) const UFFDIO_ZEROPAGE: i32 = _IOWR::<uffdio_zeropage>(UFFDIO as u32, _UFFDIO_ZEROPAGE as u32);

pub(super) const UFFDIO_WRITEPROTECT: i32 = _IOWR::<uffdio_writeprotect>(UFFDIO as u32, _UFFDIO_WRITEPROTECT as u32);
