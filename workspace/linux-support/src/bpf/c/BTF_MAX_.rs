// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Maximum number of type identifier.
pub(crate) const BTF_MAX_TYPE: usize = 0x000FFFFF;

/// Maximum offset into the string section.
pub(crate) const BTF_MAX_NAME_OFFSET: u32 = 0x00FFFFFF;

/// Maximum number of `struct`, `union` or `enum` members or the maximum number of `func` arguments.
pub(crate) const BTF_MAX_VLEN: usize = 0xFFFF;
