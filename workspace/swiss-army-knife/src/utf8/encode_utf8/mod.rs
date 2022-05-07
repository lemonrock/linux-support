// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::a_to_z::_0;
use crate::a_to_z::A;
use crate::a_to_z::Percent;
use crate::get_unchecked::GetUnchecked;
use crate::non_zero::new_non_null;
use crate::unreachable_code_const;
use crate::vec::VecExt;
use std::borrow::Cow;
use std::collections::TryReserveError;
use std::mem::size_of;
use std::ptr::NonNull;
use super::x80;
use super::ValidatedDecodeUtf8Sequences;
use super::utf8_sequence::Utf8Sequence;
use super::utf8_sequence::Utf8Sequence1;
use super::utf8_sequence::Utf8Sequence2;
use super::utf8_sequence::Utf8Sequence3;
use super::utf8_sequence::Utf8Sequence4;
use super::utf8_sequence::Utf8SequenceEnum;
use super::utf8_sequence::Utf8SequenceAndCharacter;


include!("encode_utf8_callback.rs");
include!("MAX_ONE_B.rs");
include!("MAX_THREE_B.rs");
include!("MAX_TWO_B.rs");
include!("PercentEncodedUtf8ByteSize.rs");
include!("PercentEncodeUtf8.rs");
