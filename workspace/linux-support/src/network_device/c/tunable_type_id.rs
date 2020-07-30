// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum tunable_type_id
{
	ETHTOOL_TUNABLE_UNSPEC = 0,
	ETHTOOL_TUNABLE_U8 = 1,
	ETHTOOL_TUNABLE_U16 = 2,
	ETHTOOL_TUNABLE_U32 = 3,
	ETHTOOL_TUNABLE_U64 = 4,
	ETHTOOL_TUNABLE_STRING = 5,
	ETHTOOL_TUNABLE_S8 = 6,
	ETHTOOL_TUNABLE_S16 = 7,
	ETHTOOL_TUNABLE_S32 = 8,
	ETHTOOL_TUNABLE_S64 = 9,
}
