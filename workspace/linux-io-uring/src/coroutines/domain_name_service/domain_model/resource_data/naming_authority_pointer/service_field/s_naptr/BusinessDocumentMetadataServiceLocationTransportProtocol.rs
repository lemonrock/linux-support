// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Defined in OASIS standard: [Business Document Metadata Service Location Version 1.0](https://docs.oasis-open.org/bdxr/BDX-Location/v1.0/BDX-Location-v1.0.html).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BusinessDocumentMetadataServiceLocationTransportProtocol
{
	/// Collaboration-Protocol Profile and Agreement (`CPPA`).
	///
	/// There are at least three versions.
	CPPA,
	
	/// Service Metadata Publishing, `SMP`.
	///
	/// See [Service Metadata Publishing (SMP) Version 1.0](https://docs.oasis-open.org/bdxr/bdx-smp/v1.0/bdx-smp-v1.0.html).
	///
	/// Related to [PEPPOL Transport Infrastructure Service Metadata Locator (SML)](https://www.oasis-open.org/committees/download.php/47488/ICT-Transport-SML_Service_Specification-101.pdf).
	/// PEPPOL stands for Pan-European Public Procurement Online.
	///
	/// Since [Business Document Metadata Service Location Version 1.0](https://docs.oasis-open.org/bdxr/BDX-Location/v1.0/BDX-Location-v1.0.html) was writtenm a version 2.0 has been developed.
	SMP,
}
