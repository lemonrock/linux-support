// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PciDeviceDiagnostics
{
	pub details: DiagnosticUnobtainableResult<PciDeviceDetails>,
	
	pub resources: DiagnosticUnobtainableResult<BTreeMap<u8, ResourceEntry>>,
}

impl PciDeviceDiagnostics
{
	fn gather(sys_path: &SysPath, pci_device_address: PciDeviceAddress) -> Option<Self>
	{
		let pci_device = PciDevice::new(pci_device_address, sys_path);
		
		if unlikely!(pci_device.is_pci_bus())
		{
			return None
		}
		
		Some
		(
			Self
			{
				details: catch_unwind(|| pci_device.details()).map_err(|_| DiagnosticUnobtainable(format!("Obtaining PCI device details panicked"))),
				resources: match pci_device.resources()
				{
					Err(error) => Err(DiagnosticUnobtainable::from(error)),
					
					Ok(resources) => Ok(resources.deref().clone())
				},
			}
		)
	}
}
