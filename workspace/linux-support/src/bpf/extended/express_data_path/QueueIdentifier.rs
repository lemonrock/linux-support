// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue identifier.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueIdentifier(u32);

impl QueueIdentifier
{
	// `xsk_create_bpf_maps`.
	// `xsk->xsks_map_fd`.
	/// A suitable map.
	pub fn create_express_data_path_redirect_socket_array_map(network_interface_name: NetworkInterfaceName, map_file_descriptors: &mut FileDescriptorLabelsMap<MapFileDescriptor>, access_permissions: ExpressDataPathAccessPermissions, numa_node: Option<NumaNode>) -> Result<Option<ExpressDataPathRedirectSocketArrayMap>, CreateExpressDataPathRedirectSocketMapError>
	{
		let maximum_entries = match Self::get_maximum_queues(network_interface_name)?
		{
			None => return Ok(None),
			Some(maximum_queues) => MaximumEntries::new(maximum_queues)
		};
		
		let map_name = MapName::try_from(b"xsks_map" as &[u8]).unwrap();
		
		Ok(Some(ExpressDataPathRedirectSocketArrayMap::new_xdp_redirect_socket_array(map_file_descriptors, &map_name, None, maximum_entries, access_permissions, numa_node)?))
	}
	
	#[inline(always)]
	fn get_maximum_queues(network_interface_name: NetworkInterfaceName) -> Result<Option<NonZeroU32>, NetworkDeviceInputOutputControlError<Infallible>>
	{
		NetworkDeviceSocketFileDescriptor::maximum_number_of_channels(network_interface_name)
	}
}
