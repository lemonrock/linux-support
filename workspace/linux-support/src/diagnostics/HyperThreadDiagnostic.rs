// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HyperThreadDiagnostic
{
	pub numa_node: DiagnosticUnobtainableResult<Option<NumaNode>>,
	
	pub is_online: DiagnosticUnobtainableResult<bool>,
	
	pub siblings: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub thread_siblings: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub level1_cache_hyper_thread_siblings_including_self: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub level1_cache_hyper_thread_siblings_excluding_self: DiagnosticUnobtainableResult<HyperThreads>,
	
	pub underlying_hardware_physical_core_identifier: DiagnosticUnobtainableResult<u16>,
	
	pub underlying_hardware_physical_socket_identifier: DiagnosticUnobtainableResult<u16>,
}

impl HyperThreadDiagnostic
{
	fn gather(sys_path: &SysPath, hyper_thread: HyperThread) -> Self
	{
		#[inline(always)]
		fn wrap_panic<R>(sys_path: &SysPath, callback: impl FnOnce(&SysPath) -> R) -> DiagnosticUnobtainableResult<R>
		{
			catch_unwind(AssertUnwindSafe(|| callback(sys_path))).map_err(|_| DiagnosticUnobtainable(format!("Panicked")))
		}
		
		Self
		{
			numa_node: wrap_panic(sys_path, |sys_path| hyper_thread.numa_node(sys_path)),
			
			is_online: hyper_thread.is_online(sys_path).map_err(DiagnosticUnobtainable::from),
			
			siblings: hyper_thread.siblings(sys_path).map_err(DiagnosticUnobtainable::from),
			
			thread_siblings: hyper_thread.thread_siblings(sys_path).map_err(DiagnosticUnobtainable::from),
			
			level1_cache_hyper_thread_siblings_including_self: hyper_thread.level1_cache_hyper_thread_siblings_including_self(sys_path).map_err(DiagnosticUnobtainable::from),
			
			level1_cache_hyper_thread_siblings_excluding_self: hyper_thread.level1_cache_hyper_thread_siblings_excluding_self(sys_path).map_err(DiagnosticUnobtainable::from),
			
			underlying_hardware_physical_core_identifier: hyper_thread.underlying_hardware_physical_core_identifier(sys_path).map_err(DiagnosticUnobtainable::from),
			
			underlying_hardware_physical_socket_identifier: hyper_thread.underlying_hardware_physical_socket_identifier(sys_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
