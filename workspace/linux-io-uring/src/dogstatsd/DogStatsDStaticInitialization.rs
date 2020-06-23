// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Static initialization.
pub struct DogStatsDStaticInitialization;

impl AdditionalLoggingConfiguration for DogStatsDStaticInitialization
{
	fn configure(&mut self, host_name: Option<&LinuxKernelHostName>, domain_name: Option<&LinuxKernelDomainName>, internet_protocol_addresses: &[IpAddr], process_name: &ProcessName) -> Result<(), Box<dyn error::Error + 'static>>
	{
		match host_name
		{
			None => Label::initialize_host_name(&LinuxKernelHostName::placeholder()),
			Some(host_name) => Label::initialize_host_name(host_name),
		}
		
		match domain_name
		{
			None => DogStatsDTag::initialize_environment(&LinuxKernelDomainName::placeholder()),
			Some(domain_name) => DogStatsDTag::initialize_environment(domain_name),
		}
		
		Ok(())
	}
}
