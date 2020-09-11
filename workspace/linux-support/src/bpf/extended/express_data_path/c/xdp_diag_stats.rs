// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics; only in Linux since July 2020.
#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct xdp_diag_stats
{
	#[allow(missing_docs)]
	pub n_rx_dropped: u64,
	
	#[allow(missing_docs)]
	pub n_rx_invalid: u64,
	
	#[allow(missing_docs)]
	pub n_rx_full: u64,
	
	#[allow(missing_docs)]
	pub n_fill_ring_empty: u64,
	
	#[allow(missing_docs)]
	pub n_tx_invalid: u64,
	
	#[allow(missing_docs)]
	pub n_tx_ring_empty: u64,
}
