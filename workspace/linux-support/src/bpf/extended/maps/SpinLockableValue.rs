// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Spin-lockable value for hash, array and cgroup local storage.
///
/// BPF map must have valid BTF to be usable.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpinLockableValue<V: Sized + HasReflectionInformation>
{
	/// Spin lock.
	pub spin_lock: bpf_spin_lock,

	/// Value.
	pub value: V,
}

impl<V: Sized + HasReflectionInformation> HasReflection for SpinLockableValue<V>
{
	const Type: Type = Type
	{
		type_id: TypeId::of::<Self>(),
		size: size_of::<Self>() as u32,
		ident: "SpinLockableValue",
		data: Data::Struct
		(
			StructFields::Named
			(
				&[
					NamedField
					{
						unnamed: UnnamedField
						{
							type_: &bpf_spin_lock::Type,
							offset_in_bytes: 0,
						},
						ident: "spin_lock"
					},
					NamedField
					{
						unnamed: UnnamedField
						{
							type_: &V::Type,
							offset_in_bytes: size_of::<bpf_spin_lock>() as u32,
						},
						ident: "value"
					},
				]
			)
		),
	};
}
