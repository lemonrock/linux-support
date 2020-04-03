// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use self::c::*;
use super::*;
use super::character_device::CharacterDeviceFileDescriptor;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use crate::process::ProcessIdentifier;


mod c;


include!("BackspaceDelay.rs");
include!("BaudRate.rs");
include!("BitsPerByte.rs");
include!("CanonicalEchoKillCharacter.rs");
include!("CanonicalSettings.rs");
include!("CarriageReturnDelay.rs");
include!("Character.rs");
include!("CharacterSettings.rs");
include!("ControlModeFlagSettings.rs");
include!("CurrentTerminalSettings.rs");
include!("Echo.rs");
include!("FlagSetting.rs");
include!("FormFeedDelay.rs");
include!("HorizontalTabDelay.rs");
include!("InputModeFlag.rs");
include!("InputModeFlagSettings.rs");
include!("LocalModeFlagSettings.rs");
include!("MiscellaneousControlModeFlag.rs");
include!("MiscellaneousControlModeFlagSettings.rs");
include!("MiscellaneousLocalModeFlag.rs");
include!("MiscellaneousLocalModeFlagSettings.rs");
include!("MiscellaneousOutputModeFlag.rs");
include!("MiscellaneousOutputModeFlagSettings.rs");
include!("MultipleBits.rs");
include!("NewLineDelay.rs");
include!("OutputModeFlagSettings.rs");
include!("Parity.rs");
include!("SignalRaising.rs");
include!("StopBits.rs");
include!("TerminalFileDescriptor.rs");
include!("TerminalMode.rs");
include!("TerminalSettings.rs");
include!("TerminalSettingsError.rs");
include!("VerticalTabDelay.rs");
include!("WhenToChangeTerminalAttributes.rs");
