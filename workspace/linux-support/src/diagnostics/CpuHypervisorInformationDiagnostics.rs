// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuHypervisorInformationDiagnostics
{
	pub identify: Either<String, (u32, u32, u32)>,
	
	pub tsc_frequency: Option<u32>,
	
	pub apic_frequency: Option<u32>,
}

impl CpuHypervisorInformationDiagnostics
{
	//noinspection SpellCheckingInspection
	fn gather(cpu_id: &CpuId) -> Option<Self>
	{
		use self::Hypervisor::*;
		
		cpu_id.get_hypervisor_info().map(|hypervisor_info| Self
		{
			identify: match hypervisor_info.identify()
			{
				Xen => Left("Xen".to_string()),
				
				VMware => Left("VMware".to_string()),
				
				HyperV => Left("HyperV".to_string()),
				
				KVM => Left("KVM".to_string()),
				
				Unknown(ebx, ecx, edx) => Right((ebx, ecx, edx)),
			},
			
			tsc_frequency: hypervisor_info.tsc_frequency(),
			
			apic_frequency: hypervisor_info.apic_frequency(),
		})
	}
}
