#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct scmp_arg_cmp
{
	/// Zero-based argument number.
	pub arg: c_uint,

	/// Comparison operator `SCMP_CMP_*`.
	pub op: scmp_compare,

	pub datum_a: scmp_datum_t,

	pub datum_b: scmp_datum_t,
}
