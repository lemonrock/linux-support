// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Create a **FastSecureHashSet** from a list of elements.
#[macro_export(local_inner_macros)]
macro_rules! fast_secure_hash_set
{
    (@single $($x:tt)*) => (());
    
    (@count $($rest:expr),*) => (<[()]>::len(&[$(fast_secure_hash_set!(@single $rest)),*]));

    ($($key:expr,)+) =>
    {
    	fast_secure_hash_set!($($key),+)
    };
    
    ($($key:expr),*) =>
    {
        {
            let _cap = fast_secure_hash_set!(@count $($key),*);
            let mut _set = ::swiss_army_knife::hash_map_and_hash_set::FastSecureHashSet::with_capacity(_cap);
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}
