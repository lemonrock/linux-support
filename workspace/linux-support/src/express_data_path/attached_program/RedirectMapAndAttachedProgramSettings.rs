// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Settings for a redirect map and attached program.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct RedirectMapAndAttachedProgramSettings
{
	/// If a program is already attached, overwrite it.
	///
	/// This *DOES NOT* change `UpdateMode` to `Update` (as this requires opening a file descriptor; it's just too tedious).
	pub forcibly_overwrite_already_attached: bool,
	
	/// Will only work for Netronome `nfp` drivers at the time of writing.
	///
	/// `false` is safe for all drivers.
	pub device_offload: bool,
	
	/// Store the redirect map on a specific NumaNode?
	///
	/// ?Tweak to the `NumaNode` associated with the network device?
	pub redirect_map_numa_node: Option<NumaNode>,
}
