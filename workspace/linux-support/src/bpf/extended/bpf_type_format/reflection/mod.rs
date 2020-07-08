// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use std::os::raw::c_char;


include!("pointer_type.rs");
include!("primitive_type.rs");

include!("Data.rs");
include!("EnumVariantFields.rs");
include!("Field.rs");
include!("FunctionPrototype.rs");
include!("HasReflectionInformation.rs");
include!("HasReflectionInformation.c_void.rs");
include!("NamedField.rs");
include!("Type.rs");
include!("StructFields.rs");
include!("UnnamedField.rs");
include!("Variant.rs");
