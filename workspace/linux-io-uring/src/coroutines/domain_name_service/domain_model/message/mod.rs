// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::character_strings::*;
use super::extended_dns::*;
use super::response_parsing::*;
use self::header::*;
use self::query::*;
use self::resource_record::*;


pub(crate) mod header;


pub(crate) mod query;


pub(crate) mod resource_record;




include!("DataType.rs");
include!("Message.rs");
include!("MetaType.rs");
include!("TcpMessage.rs");
