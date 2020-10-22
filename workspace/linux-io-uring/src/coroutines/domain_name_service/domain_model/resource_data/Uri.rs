// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An `URI` record.
///
/// Essentially the same as a `SRV` record, but with a URI (eg `https://myserver.com:8080/some_path`), so adding a 'path' component to a `SRV` record.
#[derive(Debug)]
pub struct Uri<'message>
{
	/// Priority.
	///
	/// RFC 2782: "A client MUST attempt to contact the target host with the lowest-numbered priority it can reach; target hosts with the same priority SHOULD be tried in an order defined by the weight field".
	pub priority: Priority,

	/// Weight.
	///
	/// Indicative of load on the server at a point in time, or, more crudely, relative performance of different servers.
	///
	/// RFC 2782: "Larger weights SHOULD be given a proportionately higher probability of being selected".
	///
	/// Larger weights imply less loading.
	pub weight: Weight,

	/// Will not be empty (this is validated).
	///
	/// This is a URI.
	pub target_uri: URI<'message>,
}
