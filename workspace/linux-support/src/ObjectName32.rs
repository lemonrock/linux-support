// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


object_name!
(
	ObjectName32,
	31,
	/// A Linux EthTool string.
	///
	/// Relies on the fact that the following are all the same length:-
	///
	/// * `ETHTOOL_FWVERS_LEN`.
	/// * `ETHTOOL_BUSINFO_LEN`.
	/// * `ETHTOOL_EROMVERS_LEN`.
	/// * `ETH_GSTRING_LEN`.
);
