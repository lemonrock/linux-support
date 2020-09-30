// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn set_field_error<Field, Error: ToString, NAT: NetlinkAttributeType, F: FnOnce(&rtattr<NAT>) -> Result<Field, Error>>(field: &mut Option<Field>, message_attribute: &rtattr<NAT>, attribute: F) -> Result<(), String>
{
	*field.as_mut().ok_or(format!("field already populated; duplicate rtattr"))? = attribute(message_attribute).map_err(|error| error.to_string())?;
	Ok(())
}
