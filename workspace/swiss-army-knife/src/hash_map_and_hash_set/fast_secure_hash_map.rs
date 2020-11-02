// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Create a **FastSecureHashMap** from a list of key-value pairs.
#[macro_export(local_inner_macros)]
macro_rules! fast_secure_hash_map
{
    (@single $($x:tt)*) => (());
    
    (@count $($rest:expr),*) => (<[()]>::len(&[$(fast_secure_hash_map!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) =>
    {
    	fast_secure_hash_map!($($key => $value),+)
    };
    
    ($($key:expr => $value:expr),*) =>
    {
        {
            let _cap = fast_secure_hash_map!(@count $($key),*);
            let mut _map = ::swiss_army_knife::hash_map_and_hash_set::FastSecureHashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
