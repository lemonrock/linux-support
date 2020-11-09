// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 6402, Section 3 Extended NAPTR Service Field Format: "appln-id = 1*10DIGIT
/// Application Identifier expressed as a decimal integer without leading zeros".
/// We could, at the expense of massive amount of code generation, match application identifiers that aren't registered.
fn modern_diameter_application_services() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"aaa" => "None",
		"aaa+ap1" => "Some(ModernDiameterApplicationIdentifier::_1)",
		"aaa+ap2" => "Some(ModernDiameterApplicationIdentifier::_2)",
		"aaa+ap3" => "Some(ModernDiameterApplicationIdentifier::_3)",
		"aaa+ap4" => "Some(ModernDiameterApplicationIdentifier::_4)",
		"aaa+ap5" => "Some(ModernDiameterApplicationIdentifier::_5)",
		"aaa+ap6" => "Some(ModernDiameterApplicationIdentifier::_6)",
		"aaa+ap7" => "Some(ModernDiameterApplicationIdentifier::_7)",
		"aaa+ap8" => "Some(ModernDiameterApplicationIdentifier::_8)",
		"aaa+ap9" => "Some(ModernDiameterApplicationIdentifier::_9)",
		"aaa+ap16777250" => "Some(ModernDiameterApplicationIdentifier::_16777250)",
		"aaa+ap16777251" => "Some(ModernDiameterApplicationIdentifier::_16777251)",
		"aaa+ap16777264" => "Some(ModernDiameterApplicationIdentifier::_16777264)",
		"aaa+ap16777267" => "Some(ModernDiameterApplicationIdentifier::_16777267)",
		"aaa+ap16777281" => "Some(ModernDiameterApplicationIdentifier::_16777281)",
		"aaa+ap16777282" => "Some(ModernDiameterApplicationIdentifier::_16777282)",
		"aaa+ap16777283" => "Some(ModernDiameterApplicationIdentifier::_16777283)",
		"aaa+ap16777284" => "Some(ModernDiameterApplicationIdentifier::_16777284)",
		"aaa+ap16777285" => "Some(ModernDiameterApplicationIdentifier::_16777285)",
		"aaa+ap16777286" => "Some(ModernDiameterApplicationIdentifier::_16777286)",
		"aaa+ap16777287" => "Some(ModernDiameterApplicationIdentifier::_16777287)",
		"aaa+ap16777288" => "Some(ModernDiameterApplicationIdentifier::_16777288)",
		"aaa+ap16777289" => "Some(ModernDiameterApplicationIdentifier::_16777289)",
		"aaa+ap16777290" => "Some(ModernDiameterApplicationIdentifier::_16777290)",
		"aaa+ap4294967295" => "Some(ModernDiameterApplicationIdentifier::_4294967295)",
	}
}
