// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
enum DriverProfileChoice
{
	LinuxOnly
	{
		/// This is the Linux in-tree driver.
		linux: &'static DriverProfile,
	},

	LinuxOrIntel
	{
		/// This is the Linux in-tree driver.
		linux: &'static DriverProfile,
		
		/// This is Intel's out-of-tree driver found at <https://sourceforge.net/p/e1000>.
		intel: &'static DriverProfile,
	}
}

impl DriverProfileChoice
{
	/// * `driver_name` is the ethtool driver name; often, but not always, this is the same as the Linux kernel module name that provides the driver.
	/// * `driver_version` is the ethtool driver name; often, but not always, this is the same as the Linux kernel module version that provides the driver.
	///
	/// Note that newer Linux kernel versions as of 2020 are moving to have the `driver_version` (and the Linux kernel module version) default to `UTS_RELEASE` (`LinuxKernelVersion.release`).
	#[inline(always)]
	fn find_driver_profile(linux_kernel_version: &LinuxKernelVersion, driver_name: &ObjectName32, driver_version: &ObjectName32, pci_vendor_and_device: PciVendorAndDevice) -> Option<&'static DriverProfile>
	{
		use self::DriverProfileChoice::*;
		
		lazy_static!
		{
    		static ref DriverProfiles: DriverProfilesMap = DriverProfileChoice::driver_profiles_map();
		};
		
		let key: DriverProfileKey = (driver_name.as_ref(), pci_vendor_and_device);
		
		let driver_profiles_map: &DriverProfilesMap = DriverProfiles.deref();
		driver_profiles_map.get(&key).map(|driver_profile_fork| match driver_profile_fork
		{
			&LinuxOnly { linux } => linux,
			
			&LinuxOrIntel { linux, intel } => if Self::is_intel_fork(linux_kernel_version, driver_name, driver_version)
			{
				intel
			}
			else
			{
				linux
			},
		})
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn driver_profiles_map() -> DriverProfilesMap
	{
		lazy_static!
		{
			static ref amazon_ena: DriverProfile = DriverProfile::amazon_ena();
			static ref intel_ixgbevf_linux_fork: DriverProfile = DriverProfile::intel_ixgbevf_linux_fork();
			static ref intel_ixgbevf_intel_fork_x540_or_earlier: DriverProfile = DriverProfile::intel_ixgbevf_intel_fork_x540_or_earlier();
			static ref intel_ixgbevf_intel_fork_x550_or_later: DriverProfile = DriverProfile::intel_ixgbevf_intel_fork_x550_or_later();
			static ref virtio_net: DriverProfile = DriverProfile::virtio_net();
		}
		
		hashmap!
		[
			(Self::ena, PciVendorAndDevice::Amazon_Ena_RESRV0) => Self::linux_only(&amazon_ena),
			(Self::ena, PciVendorAndDevice::Amazon_Ena_PF) => Self::linux_only(&amazon_ena),
			(Self::ena, PciVendorAndDevice::Amazon_Ena_LLQ_PF) => Self::linux_only(&amazon_ena),
			(Self::ena, PciVendorAndDevice::Amazon_Ena_VF) => Self::linux_only(&amazon_ena),
			(Self::ena, PciVendorAndDevice::Amazon_Ena_LLQ_VF) => Self::linux_only(&amazon_ena),
			
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_82599_VF) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x540_or_earlier),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_82599_VF_HV) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x540_or_earlier),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X540_VF) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x540_or_earlier),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X540_VF_HV) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x540_or_earlier),
			
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X550_VF) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x550_or_later),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X550_VF_HV) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x550_or_later),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X550EM_X_VF) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x550_or_later),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X550EM_X_VF_HV) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x550_or_later),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X550EM_A_VF) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x550_or_later),
			(Self::ixgbevf, PciVendorAndDevice::Intel_Ixgbe_Virtual_X550EM_A_VF_HV) => Self::linux_or_intel(&intel_ixgbevf_linux_fork, &intel_ixgbevf_intel_fork_x550_or_later),
			
			(Self::virtio_net, PciVendorAndDevice::VirtIO_Network) => Self::linux_only(&virtio_net),
			(Self::virtio_net, PciVendorAndDevice::VirtIO_NewNetwork) => Self::linux_only(&virtio_net),
		]
	}
	
	/// This logic currently works only for the Intel `ixgbevf` driver.
	///
	/// In the future it can be modified to work for other Intel drivers.
	///
	/// Linux now defaults version with `UTS_RELEASE` as of 27/01/2020 for network drivers and their supplying modules that do not specify a version.
	/// After 26/06/2020, Linux reports `UTS_RELEASE` as the version for its fork Intel ethernet drivers when the versioning overrides for both network drivers and their supplying modules was removed.
	///
	/// * `ixgbevf`:-
	/// 	* Before 26/06/2020,, the Linux fork ethtool driver version seemed to have a release tag suffix of `-k`:-
	/// 		* `4.1.0-k`: after 14/06/2017.
	/// 		* `3.2.2-k`: after 22/07/2016.
	/// 		* `2.12.1-k`: after 18/01/2014.
	/// 		* `2.11.3-k`: after 13/11/2013.
	///
	/// 	* Whereas the Intel forks from <https://sourceforge.net/projects/e1000/files/ixgbevf%20stable/> do not:-
	/// 		* `4.9.3`: after 29/09/2020 (suitable for Linux 5.8).
	/// 		* `4.8.1`: after 16/07/2020.
	/// 		* `4.7.1`: after 13/05/2020.
	/// 		* `4.6.2`: after 05/11/2019 (sic).
	/// 		* `4.6.3`: after 28/06/2019.
	/// 		* `4.6.1`: after 26/04/2019.
	/// 		* `4.5.3`: after 09/01/2019.
	/// 		* ...
	/// 		* `4.1.2`: after 18/05/2017.
	/// 		* `4.0.3`: after 01/02/2017.
	/// 		* ...
	/// 		* `3.3.2`: after 13/12/2016.
	/// 		* `3.2.2`: after 06/06/2016.
	/// 		* `3.1.2`: after 05/01/2016.
	#[doc(hidden)]
	#[inline(always)]
	fn is_intel_fork(linux_kernel_version: &LinuxKernelVersion, driver_name: &ObjectName32, driver_version: &ObjectName32) -> bool
	{
		let driver_name_bytes: &[u8] = driver_name.as_ref();
		match driver_name_bytes
		{
			Self::ixgbevf =>
			{
				let driver_version_bytes: &[u8] = driver_version.as_ref();
				if &linux_kernel_version.release[..] == driver_version_bytes
				{
					true
				}
				else if driver_version_bytes.ends_with(b"-k")
				{
					true
				}
				else
				{
					false
				}
			},
			
			_ => false,
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	const fn linux_only(linux: &'static DriverProfile) -> Self
	{
		DriverProfileChoice::LinuxOnly
		{
			linux
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	const fn linux_or_intel(linux: &'static DriverProfile, intel: &'static DriverProfile) -> Self
	{
		DriverProfileChoice::LinuxOrIntel
		{
			linux,
			
			intel,
		}
	}
	
	#[doc(hidden)]
	const ena: &'static [u8] = b"ena";
	
	#[doc(hidden)]
	const ixgbevf: &'static [u8] = b"ixgbevf";
	
	#[doc(hidden)]
	const virtio_net: &'static [u8] = b"virtio_net";
}
