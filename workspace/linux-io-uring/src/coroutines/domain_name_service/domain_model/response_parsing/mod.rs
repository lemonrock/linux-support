// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


pub(crate) mod resource_record_visitors;


include!("Answer.rs");
include!("AnswerExistence.rs");
include!("AuthoritativeOrAuthenticatedOrNeither.rs");
include!("CanonicalNameChain.rs");
include!("DuplicateResourceRecordResponseParsing.rs");
include!("NoDataResponseType.rs");
include!("NoDomainResponseType.rs");
include!("ResponseParsingState.rs");
include!("ResponseRecordSectionsParser.rs");
