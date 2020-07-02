// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Offset<'de, OV: OffsetValue>
{
	/// Known value.
	Known(OV),
	
	/// Name to to be resolved at load time.
	Named(Name<'de>),
}

impl<'de, OV: OffsetValue> From<OV> for Offset<'de, OV>
{
	#[inline(always)]
	fn from(value: OV) -> Self
	{
		Offset::Known(value)
	}
}

impl<'de, OV: OffsetValue, V: Into<Name<'de>>> From<V> for Offset<'de, OV>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Offset::Named(value.into())
	}
}

impl<'de, OV: OffsetValue> Deserialize<'de> for Offset<'de, OV>
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct OffsetVisitor;

		impl<'de, OV: OffsetValue> Visitor<'de> for OffsetVisitor<OV>
		{
			type Value = Offset<'de, OV>;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("an integer or a string")
			}
			
			#[inline(always)]
			fn visit_i8<E: de::Error>(self, value: i8) -> Result<Self::Value, E>
			{
				Self::visit_i128(i128::from(value))
			}
			
			#[inline(always)]
			fn visit_i16<E: de::Error>(self, value: i16) -> Result<Self::Value, E>
			{
				Self::visit_i128(i128::from(value))
			}
			
			#[inline(always)]
			fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E>
			{
				Self::visit_i128(i128::from(value))
			}
			
			#[inline(always)]
			fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E>
			{
				Self::visit_i128(i128::from(value))
			}
			
			#[inline(always)]
			fn visit_i128<E: de::Error>(self, value: i128) -> Result<Self::Value, E>
			{
				match OV::from_i128(value)
				{
					Ok(offset_value) => Ok(Offset::from(offset_value)),
					
					Err(()) => Err(E::custom(format!("value `{}` is out of range", value))),
				}
			}
			
			#[inline(always)]
			fn visit_u8<E: de::Error>(self, value: u8) -> Result<Self::Value, E>
			{
				Self::visit_u128(u128::from(value))
			}
			
			#[inline(always)]
			fn visit_u16<E: de::Error>(self, value: u16) -> Result<Self::Value, E>
			{
				Self::visit_u128(u128::from(value))
			}
			
			#[inline(always)]
			fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E>
			{
				Self::visit_u128(u128::from(value))
			}
			
			#[inline(always)]
			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E>
			{
				Self::visit_u128(u128::from(value))
			}
			
			#[inline(always)]
			fn visit_u128<E: de::Error>(self, value: u128) -> Result<Self::Value, E>
			{
				match OV::from_u128(value)
				{
					Ok(offset_value) => Ok(Offset::from(offset_value)),
					
					Err(()) => Err(E::custom(format!("value `{}` is out of range", value))),
				}
			}
			
			#[inline(always)]
			fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E>
			{
				Ok(Offset::from(v.to_string()))
			}
			
			#[inline(always)]
			fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E>
			{
				Ok(Offset::from(v))
			}
			
			#[inline(always)]
			fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E>
			{
				Ok(Offset::from(v))
			}
		}
		
		deserializer.deserialize_any(OffsetVisitor::<OV>)
	}
}

impl Serialize for Offset
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		use self::Offset::*;
		match self
		{
			&Known(value) => serializer.serialize_i16(value),
			&Named(Name(ref name)) => serializer.serialize_str(name),
		}
	}
}
