// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a locator pointer along with its preference.
///
/// Used in a `LP` record.
///
/// Must not be the same as the `Name` of the resource record it is associated with (this is validated before being passed to `ResourceRecordVisitor.LP()`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LocatorPointer<'label, N: Name<'label>>(N, PhantomData<&'label ()>);
