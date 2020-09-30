// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn set_address_field<IPA: InternetProtocolAddress, F: FnOnce(&rtattr<IFA>) -> &[u8]>(field: &mut Option<IPA>, message_attribute: &rtattr<IFA>, attribute: F) -> Result<(), String>
{
	set_field_error(field, message_attribute, |message_attribute| IPA::from_bytes(attribute(message_attribute)))
}
