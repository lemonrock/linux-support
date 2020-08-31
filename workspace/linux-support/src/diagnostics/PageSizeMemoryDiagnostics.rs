// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PageSizeMemoryDiagnostics
{
	pub defaults: DefaultPageSizeAndHugePageSizes,
	
	pub huge_page_size_diagnostics: HashMap<HugePageSize, HugePageSizeMemoryDiagnostic>,
}

impl PageSizeMemoryDiagnostics
{
	fn gather(sys_path: &SysPath, proc_path: &ProcPath) -> DiagnosticUnobtainableResult<Self>
	{
		let defaults = DefaultPageSizeAndHugePageSizes::new(sys_path, proc_path).map_err(DiagnosticUnobtainable::from)?;
		
		Ok
		(
			Self
			{
				defaults,
				
				huge_page_size_diagnostics:
				{
					let supported_huge_page_sizes = defaults.supported_huge_page_sizes();
					let mut huge_page_size_diagnostics = HashMap::with_capacity(supported_huge_page_sizes.len());
					for huge_page_size in supported_huge_page_sizes
					{
						let huge_page_size = *huge_page_size;
						huge_page_size_diagnostics.insert(huge_page_size, HugePageSizeMemoryDiagnostic::gather(sys_path, huge_page_size));
					}
					huge_page_size_diagnostics
				},
			}
		)
	}
}
