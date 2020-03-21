/// Errors when parsing a statistic.
#[derive(Debug)]
pub enum StatisticsParseError
{
	/// Input error.
	Input(io::Error),

	/// Invalid statistic name.
	InvalidStatisticName
	{
		/// Name.
		name: String,
	},

	/// Missing statistic value.
	MissingStatisticValue
	{
		/// Name.
		name: String,
	},

	/// Invalid statistic value.
	InvalidStatisticValue
	{
		/// Name.
		name: String,

		/// Value.
		value: String,

		/// Cause.
		cause: ParseIntError,
	},

	/// Missing statistic for number of living descendants.
	StatisticNumberOfLivingDescendantsMissing,

	/// Missing statistic for number of dying descendants.
	StatisticNumberOfDyingDescendantsMissing,
}

impl Display for StatisticsParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for StatisticsParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::StatisticsParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&InvalidStatisticName { .. } => None,

			&MissingStatisticValue { .. } => None,

			&InvalidStatisticValue { ref cause, .. } => Some(cause),

			&StatisticNumberOfLivingDescendantsMissing => None,

			&StatisticNumberOfDyingDescendantsMissing => None,
		}
	}
}

impl From<io::Error> for StatisticsParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		StatisticsParseError::Input(value)
	}
}
