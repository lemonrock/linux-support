// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! validate_number_of_entries_in_the_question_section_is_one
{
	($message_header: ident) =>
	{
		{
			let number_of_entries_in_the_question_section = $message_header.number_of_entries_in_the_question_section();
			if unlikely!(number_of_entries_in_the_question_section != 1)
			{
				return Err(ResponseDoesNotContainExactlyOneQuestion(number_of_entries_in_the_question_section))
			}
		}
	}
}
