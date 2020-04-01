// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is an iterator of process identifiers (pid)s.
///
/// It may legitimately contain duplicates, due to pid recycling occurring whilst parsing the cgroup file which provides the list.
/// Individual entries may no longer be processes despite being provided by this iterator.
#[derive(Debug)]
pub struct ProcessIdentifiersIterator(Split<BufReader<File>>);

impl Iterator for ProcessIdentifiersIterator
{
	type Item = Result<ProcessIdentifier, ProcessIdentifiersIteratorParseError>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		use self::ProcessIdentifiersIteratorParseError::*;

		match self.0.next()
		{
			None => None,

			Some(Err(io_error)) => Some(Err(Input(io_error))),

			Some(Ok(bytes)) => Some(ProcessIdentifier::parse_decimal_number(&bytes).map_err(|parse_number_error| CouldNotParseProcessIdentifier(parse_number_error))),
		}
	}
}
