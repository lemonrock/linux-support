// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinuxKernelDiagnostic
{
	pub host_name: DiagnosticUnobtainable<Option<LinuxKernelHostName>>,
	
	pub domain_name: DiagnosticUnobtainable<Option<LinuxKernelDomainName>>,

	pub version: DiagnosticUnobtainableResult<LinuxKernelVersion>,

	pub modules: DiagnosticUnobtainableResult<LinuxKernelModulesList>,

	pub lock_down_state: DiagnosticUnobtainableResult<LockDownState>,

	pub command_line_parameters: DiagnosticUnobtainableResult<LinuxKernelCommandLineParameters>,
}

impl LinuxKernelDiagnostic
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath) -> Self
	{
		Self
		{
			host_name: LinuxKernelHostName::parse(proc_path).map_err(DiagnosticUnobtainable::from),
			domain_name: LinuxKernelDomainName::parse(proc_path).map_err(DiagnosticUnobtainable::from),
			version: LinuxKernelVersion::parse(proc_path).map_err(DiagnosticUnobtainable::from),
			modules: LinuxKernelModulesList::parse(proc_path).map_err(DiagnosticUnobtainable::from),
			lock_down_state: LockDownState::current(sys_path).map_err(DiagnosticUnobtainable::from),
			command_line_parameters: LinuxKernelCommandLineParameters::parse(proc_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
