// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! validate_authentic_answers_do_not_have_authoritative_data_bit_set
{
	($message_header: ident) =>
	{
		{
			let is_authoritative_answer = $message_header.authoritative_answer();
			let is_authenticated_data = $message_header.authentic_data();

			if unlikely!(is_authoritative_answer)
			{
				if unlikely!(is_authenticated_data)
				{
					return Err(ResponseWasAuthoritativeButHasTheAuthoritativeDataBitSet)
				}
			}
			(is_authoritative_answer, is_authenticated_data)
		}
	}
}
