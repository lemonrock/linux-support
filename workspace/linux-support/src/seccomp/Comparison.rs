// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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
