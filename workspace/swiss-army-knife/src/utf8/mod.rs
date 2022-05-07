// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use byte_provider::ByteProvider;
pub use byte_provider::InvalidUtf8ParseError;
pub use byte_provider::PercentDecodeError;
use byte_provider::BytesByteProvider;
use byte_provider::PercentEncodedByteProvider;
use crate::a_to_z::A;
use crate::a_to_z::Z;
use crate::non_zero::new_non_null;
use crate::non_zero::new_non_zero_usize;
use encode_utf8::encode_utf8_callback;
use encode_utf8::PercentEncodeUtf8;
use utf8_sequence::Utf8Sequence;
use utf8_sequence::Utf8SequenceAndCharacter;
use utf8_sequence::Utf8SequenceEnum;
use std::borrow::Cow;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::fmt::Debug;
use std::mem::transmute;
use std::num::NonZeroUsize;


mod byte_provider;


mod encode_utf8;


/// UTF-8 sequence.
pub mod utf8_sequence;


include!("encode_utf8_percent_encoded.rs");
include!("is_ascii_byte.rs");
include!("is_ascii_character.rs");
include!("StringUtf8Encoding.rs");
include!("to_lower_case_ascii_byte.rs");
include!("Utf8CharacterLength.rs");
include!("x80.rs");
include!("x80Char.rs");
include!("UnvalidatedDecodeUtf8Sequences.rs");
include!("ValidatedDecodeUtf8Sequences.rs");
