// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


static mut StaticLoggingConfiguration: StaticLoggingConfiguration = unsafe { zeroed() };

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct StaticLoggingConfiguration
{
	#[cfg(debug_assertions)] initialization_pattern: u8,
	dev_path: DevPath,
	linux_kernel_host_name: Option<LinuxKernelHostName>,
	process_name: ProcessName,
	host_name: HostName,
	application_name: ApplicationName,
	internet_protocol_addresses: Box<[IpAddr]>,
	private_enterprise_number: PrivateEnterpriseNumber,
}

impl StaticLoggingConfiguration
{
	#[cfg(debug_assertions)] const Initialized: u8 = 0xFF;
	
	#[inline(always)]
	unsafe fn configure(dev_path: &DevPath, host_name: Option<&LinuxKernelHostName>, domain_name: Option<&LinuxKernelDomainName>, internet_protocol_addresses: Box<[IpAddr]>, private_enterprise_number: PrivateEnterpriseNumber, process_name: &ProcessName) -> Result<(), PrintableAsciiCharacterPushError>
	{
		debug_assert_ne!(StaticLoggingConfiguration.initialization_pattern, Self::Initialized);
		
		((&mut StaticLoggingConfiguration) as *mut StaticLoggingConfiguration).write
		(
			Self
			{
				#[cfg(debug_assertions)] initialization_pattern: Self::Initialized,
				dev_path: dev_path.clone(),
				linux_kernel_host_name: host_name.cloned(),
				process_name: process_name.clone(),
				host_name: HostName::new(host_name, domain_name)?,
				application_name: ApplicationName::new_from_process_name(process_name)?,
				internet_protocol_addresses,
				private_enterprise_number
			}
		);
		
		unsafe { LocalSyslogSocket::configure_per_thread_local_syslog_socket() };
		
		Ok(())
	}
	
	#[inline(always)]
	unsafe fn instance() -> &'static Self
	{
		debug_assert_eq!(StaticLoggingConfiguration.initialization_pattern, Self::Initialized);
		
		&StaticLoggingConfiguration
	}
	
	#[inline(always)]
	fn rfc_3164_message_template(facility: KnownFacility, severity: Severity) -> Rfc3164MessageTemplate
	{
		let this = unsafe { Self::instance() };
		Rfc3164MessageTemplate::new(facility, severity, this.linux_kernel_host_name.as_ref(), &this.process_name)
	}
	
	#[inline(always)]
	fn rfc_5424_message_template(facility: KnownFacility, severity: Severity, message_identifier: &MessageIdentifier) -> Rfc5424MessageTemplate
	{
		let this = unsafe { Self::instance() };
		Rfc5424MessageTemplate::new(facility, severity, &this.host_name, &this.application_name, message_identifier, &this.internet_protocol_addresses, &this.private_enterprise_number)
	}
}
