// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonRootCgroupVersion2HugetlbDiagnostics
{
	pub hugetlb_current: DiagnosticUnobtainableResult<Option<usize>>,
	
	pub hugetlb_reserved_maximum: DiagnosticUnobtainableResult<Option<MaximumNumber<usize>>>,
	
	pub hugetlb_event_statistics: DiagnosticUnobtainableResult<HugetlbEventStatistics>,
	
	pub hugetlb_event_statistics_local: DiagnosticUnobtainableResult<HugetlbEventStatistics>,
}

impl NonRootCgroupVersion2HugetlbDiagnostics
{
	fn gather(mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>, supported_huge_page_sizes: &BTreeSet<HugePageSize>) -> HashMap<HugePageSize, Self>
	{
		let mut diagnostics = HashMap::with_capacity(supported_huge_page_sizes.len());
		for huge_page_size in supported_huge_page_sizes.iter()
		{
			let huge_page_size = *huge_page_size;
			let diagnostic = Self
			{
				hugetlb_current: cgroup.read_hugetlb_current(mount_point, huge_page_size).map_err(DiagnosticUnobtainable::from),
				
				hugetlb_reserved_maximum: cgroup.read_hugetlb_reserved_maximum(mount_point, huge_page_size).map_err(DiagnosticUnobtainable::from),
				
				hugetlb_event_statistics: cgroup.read_hugetlb_events(mount_point, huge_page_size).map_err(DiagnosticUnobtainable::from),
				
				hugetlb_event_statistics_local: cgroup.read_hugetlb_events_local(mount_point, huge_page_size).map_err(DiagnosticUnobtainable::from),
			};
			diagnostics.insert(huge_page_size, diagnostic);
		}
		diagnostics
	}
}
