/// Comparison operation.
///
/// Ordered in priority order.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum ComparisonOperation
{
	/// Matches when the system call argument when masked by `mask` is equal to `value`.
	MaskedEqualTo
	{
		mask: u64,
		value: u64,
	},

	EqualTo(u64),

	NotEqualTo(u64),

	LessThanOrEqualTo(u64),

	LessThan(u64),

	GreaterThanOrEqualTo(u64),

	GreaterThan(u64),
}

impl Default for ComparisonOperation
{
	#[inline(always)]
	fn default() -> Self
	{
		ComparisonOperation::EqualTo(0)
	}
}

impl ComparisonOperation
{
	#[inline(always)]
	fn add_to_scmp_arg_cmp(&self, arg: &mut scmp_arg_cmp)
	{
		#[inline(always)]
		fn set_value(arg: &mut scmp_arg_cmp, operation: scmp_compare, datum_a: scmp_datum_t)
		{
			arg.op = operation;
			arg.datum_a = datum_a
		}

		use self::ComparisonOperation::*;
		use self::scmp_compare::*;

		match self
		{
			&MaskedEqualTo { mask, value } =>
			{
				set_value(arg, SCMP_CMP_MASKED_EQ, mask);
				arg.datum_b = value
			}

			&EqualTo(value) => set_value(arg, SCMP_CMP_EQ, value),

			&NotEqualTo(value) => set_value(arg, SCMP_CMP_NE, value),

			&LessThanOrEqualTo(value) => set_value(arg, SCMP_CMP_LE, value),

			&LessThan(value) => set_value(arg, SCMP_CMP_LT, value),

			&GreaterThanOrEqualTo(value) => set_value(arg, SCMP_CMP_GE, value),

			&GreaterThan(value) => set_value(arg, SCMP_CMP_GT, value),
		}
	}
}
