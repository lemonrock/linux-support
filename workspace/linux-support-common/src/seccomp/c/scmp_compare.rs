#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum scmp_compare
{
	#[doc(hidden)]
	_SCMP_CMP_MIN = 0,

	/// Not equal to.
	SCMP_CMP_NE = 1,

	/// Less than.
	SCMP_CMP_LT = 2,

	/// Less than or equal to.
	SCMP_CMP_LE = 3,

	/// Equal to.
	SCMP_CMP_EQ = 4,

	/// Greater than or equal to.
	SCMP_CMP_GE = 5,

	/// Greater than.
	SCMP_CMP_GT = 6,

	/// Masked equality.
	SCMP_CMP_MASKED_EQ = 7,

	#[doc(hidden)]
	_SCMP_CMP_MAX,
}
