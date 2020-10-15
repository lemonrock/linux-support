// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::required::SimpleNetworkTimeProtocolClientStartArguments;
use self::message::Message;
use self::message::NetworkTimeProtocolMessage;
use self::message::domain::UnsignedTimestampFormat;
use self::validate_server_replies::UnicastSocketAddresses;
use crate::coroutines::simple_network_time_protocol_client::specific::validate_server_replies::NetworkTimeProtocolMessageServerReplyParseError;
use crate::coroutines::simple_network_time_protocol_client::specific::message::domain::LeapIndicator;
use crate::coroutines::simple_network_time_protocol_client::specific::message::ServerStratum;


pub(crate) mod message;


pub(crate) mod validate_server_replies;


include!("DogStatsDSimpleNetworkTimeProtocolClientStartArguments.rs");
include!("SimpleNetworkTimeProtocolClient.rs");
include!("SimpleNetworkTimeProtocolClientDogStatsD.rs");
include!("State.rs");
