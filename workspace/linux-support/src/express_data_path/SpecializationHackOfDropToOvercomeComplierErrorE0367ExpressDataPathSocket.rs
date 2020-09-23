// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Implementations of `Drop::drop()` forward to `SpecializationHackOfDropToOvercomeComplierErrorE0367ExpressDataPathSocket::specialization_of_drop()` which allows `drop` to be specialized.
trait SpecializationHackOfDropToOvercomeComplierErrorE0367ExpressDataPathSocket
{
	/// Allows specialization (use of `default` keyword) of `Drop::drop()` with `default fn drop(&mut self)` is not allowed by Rust for reasons that I do not know (I suspect to permit one destructor implementation).
	fn specialization_of_drop(&mut self);
}
