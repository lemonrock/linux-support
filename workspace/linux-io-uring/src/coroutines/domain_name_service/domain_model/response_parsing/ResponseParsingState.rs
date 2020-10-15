// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ResponseParsingState<'message>
{
	pub(crate) have_yet_to_see_an_answer_section_cname_resource_record: bool,
	pub(crate) have_yet_to_see_an_answer_section_dname_resource_record: bool,
	pub(crate) have_yet_to_see_a_soa_resource_record: bool,
	pub(crate) have_yet_to_see_an_edns_opt_resource_record: bool,
	pub(crate) dnssec_ok: Option<bool>,

	already_encountered: HashSet<(DataType, WithCompressionParsedName<'message>, &'message [u8])>
}

impl<'message> Default for ResponseParsingState<'message>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			have_yet_to_see_an_answer_section_cname_resource_record: true,
			have_yet_to_see_an_answer_section_dname_resource_record: true,
			have_yet_to_see_a_soa_resource_record: true,
			have_yet_to_see_an_edns_opt_resource_record: true,
			dnssec_ok: None,

			already_encountered: HashSet::with_capacity(16),
		}
	}
}

impl<'message> ResponseParsingState<'message>
{
	#[inline(always)]
	pub(crate) fn encountered(&mut self, resource_record_data_type: DataType, resource_record_name: &WithCompressionParsedName<'message>, resource_data: &'message [u8]) -> Result<(), DnsProtocolError>
	{
		let has_not_yet_been_encountered = self.already_encountered.insert((resource_record_data_type, resource_record_name.clone(), resource_data));
		if likely!(has_not_yet_been_encountered)
		{
			Ok(())
		}
		else
		{
			Err(DuplicateResourceRecord(resource_record_data_type))
		}
	}
}
