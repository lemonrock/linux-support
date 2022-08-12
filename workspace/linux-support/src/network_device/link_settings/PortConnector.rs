// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Port connector.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum PortConnector
{
	/// `TP`.
	TwistedPair
	{
		/// MDI(-X) status.
		///
		/// If the status is unknown or not applicable, the value will be `ETH_TP_MDI::ETH_TP_MDI_INVALID`.
		mdi_x_status: ETH_TP_MDI,
		
		/// MDI(-X) control.
		///
		/// If MDI(-X) control is not implemented, the value will be `ETH_TP_MDI::ETH_TP_MDI_INVALID`.
		mdi_x_control: ETH_TP_MDI,
	},
	
	/// `AUI`.
	AttachmentUnitInterface,
	
	/// `MII`.
	MediaIndependentInterface,
	
	/// Fibre.
	Fibre,
	
	/// `BNC`.
	BayonetNeillConcelman,
	
	/// `DA`.
	DirectAttachCopper,
	
	/// Other
	Other,
}
