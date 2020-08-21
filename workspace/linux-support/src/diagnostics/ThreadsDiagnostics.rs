// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct ThreadsDiagnostics
{
	pub current_rust_thread_identifier: NonZeroU64,
	
	pub current_rust_thread_name: Option<String>,
	
	pub current_thread_identifier: ThreadIdentifier,
	
	pub current_thread_name: ThreadName,
	
	pub other_thread_names: DiagnosticUnobtainableResult<HashMap<ThreadIdentifier, DiagnosticUnobtainableResult<ThreadName>>>,
}

impl ThreadsDiagnostics
{
	fn gather(proc_path: &ProcPath) -> Self
	{
		let process_identifier = ProcessIdentifierChoice::Current;
		
		let current_thread_identifier = ThreadIdentifier::default();
		
		let thread = current();
		
		let current_thread_name = ThreadName::get_current_thread_name();
		
		Self
		{
			current_rust_thread_identifier: thread.id().as_u64(),
			current_rust_thread_name: thread.name().map(str::to_owned),
			current_thread_identifier,
			current_thread_name,
			other_thread_names:
			{
				match ThreadIdentifier::for_process(proc_path, process_identifier)
				{
					Err(error) => Err(DiagnosticUnobtainable::from(error)),
					Ok(thread_identifiers) =>
					{
						let mut other_thread_names = HashMap::new();
						for thread_identifier in thread_identifiers
						{
							if thread_identifier != current_thread_identifier
							{
								other_thread_names.insert(thread_identifier, ThreadName::get_thread_name(process_identifier, thread_identifier, proc_path).map_err(DiagnosticUnobtainable::from));
							}
						}
						Ok(other_thread_names)
					}
				}
			},
		}
	}
}
