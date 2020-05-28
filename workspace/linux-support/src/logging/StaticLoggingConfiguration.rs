// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


static mut StaticLoggingConfiguration: StaticInitializedOnce<StaticLoggingConfiguration> = StaticInitializedOnce::uninitialized();

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct StaticLoggingConfiguration
{
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
	fn new(dev_path: &DevPath, host_name: Option<&LinuxKernelHostName>, domain_name: Option<&LinuxKernelDomainName>, internet_protocol_addresses: &[IpAddr], private_enterprise_number: &PrivateEnterpriseNumber, process_name: &ProcessName) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		Ok
		(
			Self
			{
				dev_path: dev_path.clone(),
				linux_kernel_host_name: host_name.cloned(),
				process_name: process_name.clone(),
				host_name: HostName::new(host_name, domain_name)?,
				application_name: ApplicationName::new_from_process_name(process_name)?,
				internet_protocol_addresses: internet_protocol_addresses.to_vec().into_boxed_slice(),
				private_enterprise_number: private_enterprise_number.clone(),
			}
		)
	}
	
	#[inline(always)]
	fn new_rfc_3164_message_template(&self, facility: KnownFacility, severity: Severity) -> Rfc3164MessageTemplate
	{
		Rfc3164MessageTemplate::new(facility, severity, self.linux_kernel_host_name.as_ref(), &self.process_name)
	}
	
	#[inline(always)]
	fn new_rfc_5424_message_template(&self, facility: KnownFacility, severity: Severity, message_identifier: &MessageIdentifier) -> Rfc5424MessageTemplate
	{
		Rfc5424MessageTemplate::new(facility, severity, &self.host_name, &self.application_name, message_identifier, &self.internet_protocol_addresses, &self.private_enterprise_number)
	}
	
	#[inline(always)]
	unsafe fn configure(self)
	{
		StaticLoggingConfiguration.initialize_once(self);
	}
	
	#[inline(always)]
	fn rfc_3164_message_template(facility: KnownFacility, severity: Severity) -> Rfc3164MessageTemplate
	{
		let this = unsafe { Self::instance() };
		this.new_rfc_3164_message_template(facility, severity)
	}
	
	#[inline(always)]
	fn rfc_5424_message_template(facility: KnownFacility, severity: Severity, message_identifier: &MessageIdentifier) -> Rfc5424MessageTemplate
	{
		let this = unsafe { Self::instance() };
		this.new_rfc_5424_message_template(facility, severity, message_identifier)
	}
	
	#[inline(always)]
	unsafe fn instance() -> &'static Self
	{
		StaticLoggingConfiguration.value()
	}
}
