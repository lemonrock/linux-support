// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A case-folded (normalized to lower case) label.
#[derive(Default, Debug, Clone)]
pub struct CaseFoldedLabel<'a>(Cow<'a, [u8]>);

impl<'a> TryFrom<Box<[u8]>> for CaseFoldedLabel<'a>
{
	type Error = CaseFoldedLabelParseError;
	
	#[inline(always)]
	fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error>
	{
		Self::validate_bytes(&value[..])?;
		Ok(Self(Cow::Owned(value.to_vec())))
	}
}

impl<'a> TryFrom<Vec<u8>> for CaseFoldedLabel<'a>
{
	type Error = CaseFoldedLabelParseError;
	
	#[inline(always)]
	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error>
	{
		Self::validate_bytes(&value[..])?;
		Ok(Self(Cow::Owned(value)))
	}
}

impl<'a> TryFrom<&'a [u8]> for CaseFoldedLabel<'a>
{
	type Error = CaseFoldedLabelParseError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		Self::validate_bytes(value)?;
		Ok(Self(Cow::Borrowed(value)))
	}
}

/// This clones the underlying data and case-folds.
impl<'a, 'message> From<ParsedLabel<'message>> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn from(value: ParsedLabel<'message>) -> Self
	{
		let length = value.len();
		let capacity = length as usize;
		let mut case_folded_bytes = Vec::with_capacity(capacity);
		
		for index in 0 .. length
		{
			unsafe { *(case_folded_bytes.as_mut_ptr().add(index as usize)) = value.get_unchecked_case_folded_byte(index) };
		}
		unsafe { case_folded_bytes.set_len(capacity) }
		Self(Cow::Owned(case_folded_bytes))
	}
}

impl<'a> PartialEq for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a> Eq for CaseFoldedLabel<'a>
{
}

impl<'a> PartialOrd for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> Ord for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl<'a> Hash for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'a, 'message> PartialEq<ParsedLabel<'message>> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &ParsedLabel<'message>) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a, 'message> PartialOrd<ParsedLabel<'message>> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &ParsedLabel<'message>) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> Label<'a> for CaseFoldedLabel<'a>
{
	#[inline(always)]
	fn len(&self) -> u8
	{
		self.0.len() as u8
	}
	
	#[inline(always)]
	fn get_unchecked_case_folded_byte(&self, index: u8) -> u8
	{
		*self.get_unchecked(index)
	}
	
	#[inline(always)]
	fn get_unchecked(&self, index: u8) -> &u8
	{
		unsafe { self.0.get_unchecked(index as usize) }
	}
}

impl<'a> CaseFoldedLabel<'a>
{
	const MaximumSize: usize = 63;
	
	/// Clones and case folds.
	#[inline(always)]
	pub fn new_cloned_and_case_folded(value: &[u8]) -> Result<Self, CaseFoldedLabelParseError>
	{
		let length = value.len();
		
		if unlikely!(length > Self::MaximumSize)
		{
			return Err(CaseFoldedLabelParseError::LabelExceeded63Bytes)
		}
		
		let mut bytes = value.to_vec();
		for byte in bytes.iter_mut()
		{
			if unlikely!(*byte == b'.')
			{
				return Err(CaseFoldedLabelParseError::LabelContainedPeriod)
			}
			*byte = case_fold_byte(byte)
		}
		Ok(Self(Cow::Owned(bytes)))
	}
	
	#[inline(always)]
	fn validate_bytes(slice: &[u8]) -> Result<(), CaseFoldedLabelParseError>
	{
		let length = slice.len();
		
		if unlikely!(length > Self::MaximumSize)
		{
			return Err(CaseFoldedLabelParseError::LabelExceeded63Bytes)
		}
		
		for index in 0 .. length
		{
			Self::validate_byte(unsafe { slice.get_unchecked(index) })?
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn validate_byte(byte: &u8) -> Result<(), CaseFoldedLabelParseError>
	{
		if unlikely!(Self::is_upper_case(byte))
		{
			Err(CaseFoldedLabelParseError::LabelContainedUppercaseBytes)
		}
		else if unlikely!(*byte == b'.')
		{
			Err(CaseFoldedLabelParseError::LabelContainedPeriod)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn is_upper_case(byte: &u8) -> bool
	{
		let byte = *byte;
		byte >= b'A' || byte <= b'Z'
	}
}

impl CaseFoldedLabel<'static>
{
	#[inline(always)]
	const fn new(label: &'static [u8]) -> Self
	{
		Self(Cow::Borrowed(label))
	}
	
	const Root: Self = Self::new(b"");
	
	const _6tisch: Self = Self::new(b"6tisch");
	
	const in_addr: Self = Self::new(b"in-addr");
	
	const ip6: Self = Self::new(b"ip6");
	
	const example: Self = Self::new(b"example");
	
	const com: Self = Self::new(b"com");
	
	const net: Self = Self::new(b"net");
	
	const org: Self = Self::new(b"org");
	
	const invalid: Self = Self::new(b"invalid");
	
	const ipv4only: Self = Self::new(b"ipv4only");
	
	const arpa: Self = Self::new(b"arpa");
	
	const local: Self = Self::new(b"local");
	
	const localhost: Self = Self::new(b"localhost");
	
	const onion: Self = Self::new(b"onion");
	
	const test: Self = Self::new(b"test");
	
	const home: Self = Self::new(b"home");
	
	const intranet: Self = Self::new(b"intranet");
	
	const internal: Self = Self::new(b"internal");
	
	const private: Self = Self::new(b"private");
	
	const corp: Self = Self::new(b"corp");
	
	const lan: Self = Self::new(b"lan");
	
	const a: Self = Self::new(b"a");
	
	const b: Self = Self::new(b"b");
	
	const c: Self = Self::new(b"c");
	
	const d: Self = Self::new(b"d");
	
	const e: Self = Self::new(b"e");
	
	const f: Self = Self::new(b"f");
	
	#[inline(always)]
	const fn nibble(label: u8) -> Self
	{
		match label
		{
			0 => Self::_0,
			
			1 => Self::_1,
			
			2 => Self::_2,
			
			3 => Self::_3,
			
			4 => Self::_4,
			
			5 => Self::_5,
			
			6 => Self::_6,
			
			7 => Self::_7,
			
			8 => Self::_8,
			
			9 => Self::_9,
			
			10 => Self::a,
			
			10 => Self::b,
			
			10 => Self::c,
			
			10 => Self::d,
			
			10 => Self::e,
			
			10 => Self::f,
			
			_ => unreachable!("label {} was not a nibble")
		}
	}
	
	#[inline(always)]
	const fn byte(label: u8) -> Self
	{
		match label
		{
			0 => Self::_0,

			1 => Self::_1,

			2 => Self::_2,

			3 => Self::_3,

			4 => Self::_4,

			5 => Self::_5,

			6 => Self::_6,

			7 => Self::_7,

			8 => Self::_8,

			9 => Self::_9,

			10 => Self::_10,
			
			11 => Self::_11,

			12 => Self::_12,

			13 => Self::_13,

			14 => Self::_14,

			15 => Self::_15,

			16 => Self::_16,

			17 => Self::_17,

			18 => Self::_18,

			19 => Self::_19,

			20 => Self::_20,

			21 => Self::_21,

			22 => Self::_22,

			23 => Self::_23,

			24 => Self::_24,

			25 => Self::_25,

			26 => Self::_26,

			27 => Self::_27,

			28 => Self::_28,

			29 => Self::_29,

			30 => Self::_30,

			31 => Self::_31,

			32 => Self::_32,

			33 => Self::_33,

			34 => Self::_34,

			35 => Self::_35,

			36 => Self::_36,

			37 => Self::_37,

			38 => Self::_38,

			39 => Self::_39,

			40 => Self::_40,

			41 => Self::_41,

			42 => Self::_42,

			43 => Self::_43,

			44 => Self::_44,

			45 => Self::_45,

			46 => Self::_46,

			47 => Self::_47,

			48 => Self::_48,

			49 => Self::_49,

			50 => Self::_50,

			51 => Self::_51,

			52 => Self::_52,

			53 => Self::_53,

			54 => Self::_54,

			55 => Self::_55,

			56 => Self::_56,

			57 => Self::_57,

			58 => Self::_58,

			59 => Self::_59,

			60 => Self::_60,

			61 => Self::_61,

			62 => Self::_62,

			63 => Self::_63,

			64 => Self::_64,

			65 => Self::_65,

			66 => Self::_66,

			67 => Self::_67,

			68 => Self::_68,

			69 => Self::_69,

			70 => Self::_70,

			71 => Self::_71,

			72 => Self::_72,

			73 => Self::_73,

			74 => Self::_74,

			75 => Self::_75,

			76 => Self::_76,

			77 => Self::_77,

			78 => Self::_78,

			79 => Self::_79,

			80 => Self::_80,

			81 => Self::_81,

			82 => Self::_82,

			83 => Self::_83,

			84 => Self::_84,

			85 => Self::_85,

			86 => Self::_86,

			87 => Self::_87,

			88 => Self::_88,

			89 => Self::_89,

			90 => Self::_90,

			91 => Self::_91,

			92 => Self::_92,

			93 => Self::_93,

			94 => Self::_94,

			95 => Self::_95,

			96 => Self::_96,

			97 => Self::_97,

			98 => Self::_98,

			99 => Self::_99,

			100 => Self::_100,

			101 => Self::_101,

			102 => Self::_102,

			103 => Self::_103,

			104 => Self::_104,

			105 => Self::_105,

			106 => Self::_106,

			107 => Self::_107,

			108 => Self::_108,

			109 => Self::_109,

			110 => Self::_110,

			111 => Self::_111,

			112 => Self::_112,

			113 => Self::_113,

			114 => Self::_114,

			115 => Self::_115,

			116 => Self::_116,

			117 => Self::_117,

			118 => Self::_118,

			119 => Self::_119,

			120 => Self::_120,

			121 => Self::_121,

			122 => Self::_122,

			123 => Self::_123,

			124 => Self::_124,

			125 => Self::_125,

			126 => Self::_126,

			127 => Self::_127,

			128 => Self::_128,

			129 => Self::_129,

			130 => Self::_130,

			131 => Self::_131,

			132 => Self::_132,

			133 => Self::_133,

			134 => Self::_134,

			135 => Self::_135,

			136 => Self::_136,

			137 => Self::_137,

			138 => Self::_138,

			139 => Self::_139,

			140 => Self::_140,

			141 => Self::_141,

			142 => Self::_142,

			143 => Self::_143,

			144 => Self::_144,

			145 => Self::_145,

			146 => Self::_146,

			147 => Self::_147,

			148 => Self::_148,

			149 => Self::_149,

			150 => Self::_150,

			151 => Self::_151,

			152 => Self::_152,

			153 => Self::_153,

			154 => Self::_154,

			155 => Self::_155,

			156 => Self::_156,

			157 => Self::_157,

			158 => Self::_158,

			159 => Self::_159,

			160 => Self::_160,

			161 => Self::_161,

			162 => Self::_162,

			163 => Self::_163,

			164 => Self::_164,

			165 => Self::_165,

			166 => Self::_166,

			167 => Self::_167,

			168 => Self::_168,

			169 => Self::_169,

			170 => Self::_170,

			171 => Self::_171,

			172 => Self::_172,

			173 => Self::_173,

			174 => Self::_174,

			175 => Self::_175,

			176 => Self::_176,

			177 => Self::_177,

			178 => Self::_178,

			179 => Self::_179,

			180 => Self::_180,

			181 => Self::_181,

			182 => Self::_182,

			183 => Self::_183,

			184 => Self::_184,

			185 => Self::_185,

			186 => Self::_186,

			187 => Self::_187,

			188 => Self::_188,

			189 => Self::_189,

			190 => Self::_190,

			191 => Self::_191,

			192 => Self::_192,

			193 => Self::_193,

			194 => Self::_194,

			195 => Self::_195,

			196 => Self::_196,

			197 => Self::_197,

			198 => Self::_198,

			199 => Self::_199,

			200 => Self::_200,

			201 => Self::_201,

			202 => Self::_202,

			203 => Self::_203,

			204 => Self::_204,

			205 => Self::_205,

			206 => Self::_206,

			207 => Self::_207,

			208 => Self::_208,

			209 => Self::_209,

			210 => Self::_210,

			211 => Self::_211,

			212 => Self::_212,

			213 => Self::_213,

			214 => Self::_214,

			215 => Self::_215,

			216 => Self::_216,

			217 => Self::_217,

			218 => Self::_218,

			219 => Self::_219,

			220 => Self::_220,

			221 => Self::_221,

			222 => Self::_222,

			223 => Self::_223,

			224 => Self::_224,

			225 => Self::_225,

			226 => Self::_226,

			227 => Self::_227,

			228 => Self::_228,

			229 => Self::_229,

			230 => Self::_230,

			231 => Self::_231,

			232 => Self::_232,

			233 => Self::_233,

			234 => Self::_234,

			235 => Self::_235,

			236 => Self::_236,

			237 => Self::_237,

			238 => Self::_238,

			239 => Self::_239,

			240 => Self::_240,

			241 => Self::_241,

			242 => Self::_242,

			243 => Self::_243,

			244 => Self::_244,

			245 => Self::_245,

			246 => Self::_246,

			247 => Self::_247,

			248 => Self::_248,

			249 => Self::_249,

			250 => Self::_250,

			251 => Self::_251,

			252 => Self::_252,

			253 => Self::_253,

			254 => Self::_254,

			255 => Self::_255,
		}
	}
	
	const _0: Self = Self::new(b"0");
	
	const _1: Self = Self::new(b"1");
	
	const _2: Self = Self::new(b"2");
	
	const _3: Self = Self::new(b"3");
	
	const _4: Self = Self::new(b"4");
	
	const _5: Self = Self::new(b"5");
	
	const _6: Self = Self::new(b"6");
	
	const _7: Self = Self::new(b"7");
	
	const _8: Self = Self::new(b"8");
	
	const _9: Self = Self::new(b"9");
	
	const _10: Self = Self::new(b"10");
	
	const _11: Self = Self::new(b"11");
	
	const _12: Self = Self::new(b"12");
	
	const _13: Self = Self::new(b"13");
	
	const _14: Self = Self::new(b"14");
	
	const _15: Self = Self::new(b"15");
	
	const _16: Self = Self::new(b"16");
	
	const _17: Self = Self::new(b"17");
	
	const _18: Self = Self::new(b"18");
	
	const _19: Self = Self::new(b"19");
	
	const _20: Self = Self::new(b"20");
	
	const _21: Self = Self::new(b"21");
	
	const _22: Self = Self::new(b"22");
	
	const _23: Self = Self::new(b"23");
	
	const _24: Self = Self::new(b"24");
	
	const _25: Self = Self::new(b"25");
	
	const _26: Self = Self::new(b"26");
	
	const _27: Self = Self::new(b"27");
	
	const _28: Self = Self::new(b"28");
	
	const _29: Self = Self::new(b"29");
	
	const _30: Self = Self::new(b"30");
	
	const _31: Self = Self::new(b"31");
	
	const _32: Self = Self::new(b"32");
	
	const _33: Self = Self::new(b"33");
	
	const _34: Self = Self::new(b"34");
	
	const _35: Self = Self::new(b"35");
	
	const _36: Self = Self::new(b"36");
	
	const _37: Self = Self::new(b"37");
	
	const _38: Self = Self::new(b"38");
	
	const _39: Self = Self::new(b"39");
	
	const _40: Self = Self::new(b"40");
	
	const _41: Self = Self::new(b"41");
	
	const _42: Self = Self::new(b"42");
	
	const _43: Self = Self::new(b"43");
	
	const _44: Self = Self::new(b"44");
	
	const _45: Self = Self::new(b"45");
	
	const _46: Self = Self::new(b"46");
	
	const _47: Self = Self::new(b"47");
	
	const _48: Self = Self::new(b"48");
	
	const _49: Self = Self::new(b"49");
	
	const _50: Self = Self::new(b"50");
	
	const _51: Self = Self::new(b"51");
	
	const _52: Self = Self::new(b"52");
	
	const _53: Self = Self::new(b"53");
	
	const _54: Self = Self::new(b"54");
	
	const _55: Self = Self::new(b"55");
	
	const _56: Self = Self::new(b"56");
	
	const _57: Self = Self::new(b"57");
	
	const _58: Self = Self::new(b"58");
	
	const _59: Self = Self::new(b"59");
	
	const _60: Self = Self::new(b"60");
	
	const _61: Self = Self::new(b"61");
	
	const _62: Self = Self::new(b"62");
	
	const _63: Self = Self::new(b"63");
	
	const _64: Self = Self::new(b"64");
	
	const _65: Self = Self::new(b"65");
	
	const _66: Self = Self::new(b"66");
	
	const _67: Self = Self::new(b"67");
	
	const _68: Self = Self::new(b"68");
	
	const _69: Self = Self::new(b"69");
	
	const _70: Self = Self::new(b"70");
	
	const _71: Self = Self::new(b"71");
	
	const _72: Self = Self::new(b"72");
	
	const _73: Self = Self::new(b"73");
	
	const _74: Self = Self::new(b"74");
	
	const _75: Self = Self::new(b"75");
	
	const _76: Self = Self::new(b"76");
	
	const _77: Self = Self::new(b"77");
	
	const _78: Self = Self::new(b"78");
	
	const _79: Self = Self::new(b"79");
	
	const _80: Self = Self::new(b"80");
	
	const _81: Self = Self::new(b"81");
	
	const _82: Self = Self::new(b"82");
	
	const _83: Self = Self::new(b"83");
	
	const _84: Self = Self::new(b"84");
	
	const _85: Self = Self::new(b"85");
	
	const _86: Self = Self::new(b"86");
	
	const _87: Self = Self::new(b"87");
	
	const _88: Self = Self::new(b"88");
	
	const _89: Self = Self::new(b"89");
	
	const _90: Self = Self::new(b"90");
	
	const _91: Self = Self::new(b"91");
	
	const _92: Self = Self::new(b"92");
	
	const _93: Self = Self::new(b"93");
	
	const _94: Self = Self::new(b"94");
	
	const _95: Self = Self::new(b"95");
	
	const _96: Self = Self::new(b"96");
	
	const _97: Self = Self::new(b"97");
	
	const _98: Self = Self::new(b"98");
	
	const _99: Self = Self::new(b"99");
	
	const _100: Self = Self::new(b"100");
	
	const _101: Self = Self::new(b"101");
	
	const _102: Self = Self::new(b"102");
	
	const _103: Self = Self::new(b"103");
	
	const _104: Self = Self::new(b"104");
	
	const _105: Self = Self::new(b"105");
	
	const _106: Self = Self::new(b"106");
	
	const _107: Self = Self::new(b"107");
	
	const _108: Self = Self::new(b"108");
	
	const _109: Self = Self::new(b"109");
	
	const _110: Self = Self::new(b"110");
	
	const _111: Self = Self::new(b"111");
	
	const _112: Self = Self::new(b"112");
	
	const _113: Self = Self::new(b"113");
	
	const _114: Self = Self::new(b"114");
	
	const _115: Self = Self::new(b"115");
	
	const _116: Self = Self::new(b"116");
	
	const _117: Self = Self::new(b"117");
	
	const _118: Self = Self::new(b"118");
	
	const _119: Self = Self::new(b"119");
	
	const _120: Self = Self::new(b"120");
	
	const _121: Self = Self::new(b"121");
	
	const _122: Self = Self::new(b"122");
	
	const _123: Self = Self::new(b"123");
	
	const _124: Self = Self::new(b"124");
	
	const _125: Self = Self::new(b"125");
	
	const _126: Self = Self::new(b"126");
	
	const _127: Self = Self::new(b"127");
	
	const _128: Self = Self::new(b"128");
	
	const _129: Self = Self::new(b"129");
	
	const _130: Self = Self::new(b"130");
	
	const _131: Self = Self::new(b"131");
	
	const _132: Self = Self::new(b"132");
	
	const _133: Self = Self::new(b"133");
	
	const _134: Self = Self::new(b"134");
	
	const _135: Self = Self::new(b"135");
	
	const _136: Self = Self::new(b"136");
	
	const _137: Self = Self::new(b"137");
	
	const _138: Self = Self::new(b"138");
	
	const _139: Self = Self::new(b"139");
	
	const _140: Self = Self::new(b"140");
	
	const _141: Self = Self::new(b"141");
	
	const _142: Self = Self::new(b"142");
	
	const _143: Self = Self::new(b"143");
	
	const _144: Self = Self::new(b"144");
	
	const _145: Self = Self::new(b"145");
	
	const _146: Self = Self::new(b"146");
	
	const _147: Self = Self::new(b"147");
	
	const _148: Self = Self::new(b"148");
	
	const _149: Self = Self::new(b"149");
	
	const _150: Self = Self::new(b"150");
	
	const _151: Self = Self::new(b"151");
	
	const _152: Self = Self::new(b"152");
	
	const _153: Self = Self::new(b"153");
	
	const _154: Self = Self::new(b"154");
	
	const _155: Self = Self::new(b"155");
	
	const _156: Self = Self::new(b"156");
	
	const _157: Self = Self::new(b"157");
	
	const _158: Self = Self::new(b"158");
	
	const _159: Self = Self::new(b"159");
	
	const _160: Self = Self::new(b"160");
	
	const _161: Self = Self::new(b"161");
	
	const _162: Self = Self::new(b"162");
	
	const _163: Self = Self::new(b"163");
	
	const _164: Self = Self::new(b"164");
	
	const _165: Self = Self::new(b"165");
	
	const _166: Self = Self::new(b"166");
	
	const _167: Self = Self::new(b"167");
	
	const _168: Self = Self::new(b"168");
	
	const _169: Self = Self::new(b"169");
	
	const _170: Self = Self::new(b"170");
	
	const _171: Self = Self::new(b"171");
	
	const _172: Self = Self::new(b"172");
	
	const _173: Self = Self::new(b"173");
	
	const _174: Self = Self::new(b"174");
	
	const _175: Self = Self::new(b"175");
	
	const _176: Self = Self::new(b"176");
	
	const _177: Self = Self::new(b"177");
	
	const _178: Self = Self::new(b"178");
	
	const _179: Self = Self::new(b"179");
	
	const _180: Self = Self::new(b"180");
	
	const _181: Self = Self::new(b"181");
	
	const _182: Self = Self::new(b"182");
	
	const _183: Self = Self::new(b"183");
	
	const _184: Self = Self::new(b"184");
	
	const _185: Self = Self::new(b"185");
	
	const _186: Self = Self::new(b"186");
	
	const _187: Self = Self::new(b"187");
	
	const _188: Self = Self::new(b"188");
	
	const _189: Self = Self::new(b"189");
	
	const _190: Self = Self::new(b"190");
	
	const _191: Self = Self::new(b"191");
	
	const _192: Self = Self::new(b"192");
	
	const _193: Self = Self::new(b"193");
	
	const _194: Self = Self::new(b"194");
	
	const _195: Self = Self::new(b"195");
	
	const _196: Self = Self::new(b"196");
	
	const _197: Self = Self::new(b"197");
	
	const _198: Self = Self::new(b"198");
	
	const _199: Self = Self::new(b"199");
	
	const _200: Self = Self::new(b"200");
	
	const _201: Self = Self::new(b"201");
	
	const _202: Self = Self::new(b"202");
	
	const _203: Self = Self::new(b"203");
	
	const _204: Self = Self::new(b"204");
	
	const _205: Self = Self::new(b"205");
	
	const _206: Self = Self::new(b"206");
	
	const _207: Self = Self::new(b"207");
	
	const _208: Self = Self::new(b"208");
	
	const _209: Self = Self::new(b"209");
	
	const _210: Self = Self::new(b"210");
	
	const _211: Self = Self::new(b"211");
	
	const _212: Self = Self::new(b"212");
	
	const _213: Self = Self::new(b"213");
	
	const _214: Self = Self::new(b"214");
	
	const _215: Self = Self::new(b"215");
	
	const _216: Self = Self::new(b"216");
	
	const _217: Self = Self::new(b"217");
	
	const _218: Self = Self::new(b"218");
	
	const _219: Self = Self::new(b"219");
	
	const _220: Self = Self::new(b"220");
	
	const _221: Self = Self::new(b"221");
	
	const _222: Self = Self::new(b"222");
	
	const _223: Self = Self::new(b"223");
	
	const _224: Self = Self::new(b"224");
	
	const _225: Self = Self::new(b"225");
	
	const _226: Self = Self::new(b"226");
	
	const _227: Self = Self::new(b"227");
	
	const _228: Self = Self::new(b"228");
	
	const _229: Self = Self::new(b"229");
	
	const _230: Self = Self::new(b"230");
	
	const _231: Self = Self::new(b"231");
	
	const _232: Self = Self::new(b"232");
	
	const _233: Self = Self::new(b"233");
	
	const _234: Self = Self::new(b"234");
	
	const _235: Self = Self::new(b"235");
	
	const _236: Self = Self::new(b"236");
	
	const _237: Self = Self::new(b"237");
	
	const _238: Self = Self::new(b"238");
	
	const _239: Self = Self::new(b"239");
	
	const _240: Self = Self::new(b"240");
	
	const _241: Self = Self::new(b"241");
	
	const _242: Self = Self::new(b"242");
	
	const _243: Self = Self::new(b"243");
	
	const _244: Self = Self::new(b"244");
	
	const _245: Self = Self::new(b"245");
	
	const _246: Self = Self::new(b"246");
	
	const _247: Self = Self::new(b"247");
	
	const _248: Self = Self::new(b"248");
	
	const _249: Self = Self::new(b"249");
	
	const _250: Self = Self::new(b"250");
	
	const _251: Self = Self::new(b"251");
	
	const _252: Self = Self::new(b"252");
	
	const _253: Self = Self::new(b"253");
	
	const _254: Self = Self::new(b"254");
	
	const _255: Self = Self::new(b"255");
}
