// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `BPF_MAP_TYPE_HASH`, `BPF_MAP_TYPE_ARRAY`, `BPF_MAP_TYPE_PERCPU_HASH`, `BPF_MAP_TYPE_PERCPU_ARRAY`, `BPF_MAP_TYPE_LRU_HASH`, `BPF_MAP_TYPE_LRU_PERCPU_HASH` and `BPF_MAP_TYPE_LPM_TRIE`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenericMap(MapFileDescriptor);



impl GenericMap
{
	pub fn lookup(&self, key: ())
	{
	}
	
	pub fn create_or_update(&self, key: (), value: ())
	{
	}
	
	pub fn delete(&self, key: ())
	{
	}
	
	pub fn next_key(&self, key: ()) -> ()
	{
	
	}
}

/*
       BPF_MAP_CREATE
              Create a map and return a file descriptor that refers to the
              map.  The close-on-exec file descriptor flag (see fcntl(2)) is
              automatically enabled for the new file descriptor.

       BPF_MAP_LOOKUP_ELEM
              Look up an element by key in a specified map and return its
              value.

       BPF_MAP_UPDATE_ELEM
              Create or update an element (key/value pair) in a specified
              map.

       BPF_MAP_DELETE_ELEM
              Look up and delete an element by key in a specified map.

       BPF_MAP_GET_NEXT_KEY
              Look up an element by key in a specified map and return the
              key of the next element.
 */
