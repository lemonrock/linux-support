// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A function definition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionPrototype
{
	/// Parameters.
	pub parameters: &'static [(&'static str, &'static Type)],
	
	/// Return type.
	pub return_type: &'static Type,

	/// Linkage.
	pub linkage: btf_func_linkage,
}

impl Display for FunctionPrototype
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::bpf_func_linkage::*;
		let linkage = match self.linkage
		{
			Static => "static",
			Global => "global",
			Extern => "extern",
		};
		
		write!(f, "{} fn {}(", linkage, self.ident)?;
		
		let mut after_first = false;
		for &(parameter_name, parameter_type) in self.parameters
		{
			if after_first
			{
				write!(f, ", ")?;
			}
			else
			{
				after_first = true;
			}
			write!(f, "{}: {}", parameter_name, parameter_type.ident)?;
		}
		
		write!(f, ") -> {}", self.return_type.ident)
	}
}
