/// Comparison.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct Comparison
{
	/// Zero based argument number, typically between 0 to 5.
	#[serde(default)] pub zero_based_argument_number: ZeroBasedArgumentNumber,

	/// Comparison operation.
	#[serde(default)] pub comparison_operation: ComparisonOperation,
}

impl Comparison
{
	#[inline(always)]
	fn to_scmp_arg_cmp(&self) -> scmp_arg_cmp
	{
		let mut arg = scmp_arg_cmp
		{
			arg: self.zero_based_argument_number as u32,
			op: scmp_compare::_SCMP_CMP_MIN,
			datum_a: 0,
			datum_b: 0,
		};
		self.comparison_operation.add_to_scmp_arg_cmp(&mut arg);
		arg
	}
}
