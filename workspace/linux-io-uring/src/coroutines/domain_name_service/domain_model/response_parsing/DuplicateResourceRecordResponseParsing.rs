// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug)]
pub(crate) struct DuplicateResourceRecordResponseParsing<'message>
{
	already_encountered: RefCell<HashSet<(DataType, ParsedName<'message>, &'message [u8])>>
}

impl<'message> DuplicateResourceRecordResponseParsing<'message>
{
	#[inline(always)]
	pub(crate) fn encountered(&mut self, resource_record_data_type: DataType, resource_record_name: &ParsedName<'message>, resource_data: &'message [u8]) -> Result<(), ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError>
	{
		let has_not_yet_been_encountered = self.already_encountered.borrow_mut().insert((resource_record_data_type, resource_record_name.clone(), resource_data));
		if likely!(has_not_yet_been_encountered)
		{
			Ok(())
		}
		else
		{
			Err(ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError::DuplicateResourceRecord(resource_record_data_type))
		}
	}
}
