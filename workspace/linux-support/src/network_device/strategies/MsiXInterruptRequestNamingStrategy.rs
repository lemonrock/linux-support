// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// * `bus_info_name` is the value as returned by Linux's `pci_name()` function. The Amazon ENA, Intel ixgbevf (either fork) and virtio net drivers it is the same as the value returned by ethtool in `ethtool_drvinfo.bus_info` and is just a `PciDeviceAddress`, formatted.
/// * `device_name` is used by virtio net; it is something like `virtio0`.
///
// pub enum MsiXInterruptRequestNamingStrategy
// {
// 	AmazonEna,
//
// 	IntelIxgbevfLinux,
//
// 	IntelIxgbevfIntel,
//
// 	VirtioNet,
// }



