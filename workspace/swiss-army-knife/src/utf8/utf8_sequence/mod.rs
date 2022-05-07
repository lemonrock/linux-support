// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::from_unchecked::FromUnchecked;
use std::fmt::Debug;
use std::char::CharTryFromError;
use std::hash::Hash;
use std::mem::discriminant;
use std::mem::transmute;
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use super::byte_provider::ByteProvider;
use super::encode_utf8::encode_utf8_callback;
use super::Utf8CharacterLength;
use super::Utf8CharacterLength::*;
use super::x80;


include!("TAG_CONT.rs");
include!("TAG_FOUR_B.rs");
include!("TAG_THREE_B.rs");
include!("TAG_TWO_B.rs");
include!("Shift6.rs");
include!("Shift12.rs");
include!("Shift18.rs");
include!("Utf8Sequence.rs");
include!("Utf8Sequence1.rs");
include!("Utf8Sequence2.rs");
include!("Utf8Sequence3.rs");
include!("Utf8Sequence4.rs");
include!("Utf8SequenceAndCharacter.rs");
include!("Utf8SequenceEnum.rs");
include!("Utf8SequenceCrate.rs");
include!("Utf8SequenceNonConst.rs");
include!("x07.rs");
include!("x0F.rs");
include!("x1F.rs");
include!("x3F.rs");
include!("xC0.rs");
include!("xE0.rs");
include!("xF0.rs");
