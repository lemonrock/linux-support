// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


include!("ConfiguredHashSettings.rs");
include!("ContextIdentifier.rs");
include!("ContextIdentifierOrCreate.rs");
include!("UnsupportedHashFunctionError.rs");


/// Usually 40 bytes; some cards use 56 bytes.
pub struct ReceiveSideScalingHashKey(Vec<u8>);

/// There must be at least entry.
pub struct ReceiveQueueWeightings(IndexMap<QueueIdentifier, ReceiveQueueWeighting>);

pub struct ReceiveQueueWeighting(u32);

/*

-X --set-rxfh-indir --rxfh
	do_srxfh
		do_srxfhindir
		
ETHTOOL_GRXRINGS
ETHTOOL_GRSSH


indirection table is called RETA
To use Toeplitz or XOR or CRC32, the feature NETIF_F_RXHASH should be set.

hkey 		xx:yy:zz:aa:bb:cc
`rxfhindir_key`
size is dictated by ethtool_rxfh.cmd=ETHTOOL_GRSSH, ethtool_rxfh.key_size
	hash keys are not supported if ethtool_rxfh.key_size is 0.
	
hfunc		set hash function name	names are in --show-rxfh
`req_hfunc_name`

equal		N
`rxfhindir_equal`
	start 	S
	`rxfhindir_start`
	Spread weights evenly starting at queue S
OR
weight		W0 W1 ..
`rxfhindir_weight`
	start 	S
	`rxfhindir_start`
	Spread weights W0, W1 starting at queue S
default		reset
	`rxfhindir_default`

contexts
	- can create using context new
	- can manipulate using context CTX
	- can delete using delete CTX
	- CTX is a u32; default is 0
	- a new context is created using ETH_RXFH_CONTEXT_ALLOC
	- #define ETH_RXFH_CONTEXT_ALLOC		0xffffffff
	- CTX 0 id the default context for normal traffic.
	
 */
