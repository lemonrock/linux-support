// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A BPF spin-lock.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct bpf_spin_lock
{
	/// Value.
	pub val: u32
}

impl HasReflectionInformation for bpf_spin_lock
{
	const Type: Type = Type
	{
		type_id: TypeId::of::<Self>(),
		size: size_of::<Self>() as u32,
		ident: "bpf_spin_lock",
		data: Data::Struct
		(
			StructFields::Named
			(
				&[
					
					NamedField
					{
						unnamed: UnnamedField
						{
							type_: &u32::Type,
							offset_in_bytes: 0,
						},
						ident: "val"
					},
				]
			)
		),
	};
}
