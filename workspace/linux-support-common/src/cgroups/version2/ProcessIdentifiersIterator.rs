/// This is an iterator of process identifiers (pid)s.
///
/// It may legitimately contain duplicates, due to pid recycling occurring whilst parsing the cgroup file which provides the list.
/// Individual entries may no longer be processes despite being provided by this iterator.
#[derive(Debug)]
pub struct ProcessIdentifiersIterator(Lines<BufReader<File>>);

impl Iterator for ProcessIdentifiersIterator
{
	type Item = Result<NonZeroU32, ProcessIdentifiersIteratorParseError>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		use self::ProcessIdentifiersIteratorParseError::*;

		match self.0.next()
		{
			None => None,

			Some(Err(io_error)) => Some(Err(Input(io_error))),

			Some(Ok(potential_process_identifier)) =>
			{
				let value: Result<u32, ParseIntError> = potential_process_identifier.parse();
				let outcome = match value
				{
					Ok(potential_process_identifier) => if unlikely!(potential_process_identifier == 0)
					{
						Err(ProcessIdentifierCanNotBeZero)
					}
					else
					{
						Ok(unsafe { NonZeroU32::new_unchecked(potential_process_identifier) })
					},

					Err(parse_int_error) => Err(CouldNotParseProcessIdentifier(parse_int_error)),
				};
				Some(outcome)
			}
		}

	}
}
