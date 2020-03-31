// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Creates a BitSet.
#[macro_export]
macro_rules! bit_set
{
    ($($element: expr),*) =>
    {
        {
            let mut _set = $crate::bit_set::BitSet::empty();
            $(
                _set.add($element);
            )*
            _set
        }
    };
}
