#!/usr/bin/env sh


set -e
set -f
set -u

register_mode()
{
		local mode="$1"

		case "$mode" in

			RO)
					registerMode="RegisterDefinition<ReadOnlyRegister<u32>, u32>"
			;;

			RW)
					registerMode="RegisterDefinition<ReadWriteRegister<u32>, u32>"
			;;

			RC)
					registerMode="RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32>"
			;;

			RCR)
					registerMode="RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32>"
			;;

			RO64)
					registerMode="RegisterDefinition<ReadOnlyRegister<u64>, u64>"
			;;

			RW64)
					registerMode="RegisterDefinition<ReadWriteRegister<u64>, u64>"
			;;

			RC64)
					registerMode="RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64>"
			;;

			RCR64)
					registerMode="RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u64>, u64>"
			;;

		esac
}

parse_indexing()
{
	local indexing="$1"
	local shortName="$2"
	local file="$3"

	case "$indexing" in

		'-')
			step=''
			min=''
			max=''
		;;

		'+'*)
			# eg +0x04*0..31
			printf '%s\n' "$indexing" | sed -e 's;^+;;g' -e 's;\*; ;g' -e 's;\.\.; ;g' >snabb-switch-parse.tmp
			IFS=' ' read -r step min max <snabb-switch-parse.tmp
			rm snabb-switch-parse.tmp
		;;


		*)
			printf 'Invalid indexing %s in %s\n' "$shortName" "$file"
			exit 1
		;;

	esac
}

reset_previous()
{
	previousShortName=''
	previousOffset=''
	previousLongName=''
	previousStep=''
	previousMin=''
	previousMax=''
	previousRegisterMode=''

	waitForNextRowBeforeWriting=true
}

store_current_as_previous()
{
	previousShortName="$shortName"
	previousOffset="$offset"
	previousLongName="$longName"
	previousStep="$step"
	previousMin="$min"
	previousMax="$max"
	previousRegisterMode="$registerMode"

	waitForNextRowBeforeWriting=false
}

write_split_array()
{
	cat <<EOF

/// ${previousLongName}.
pub const ${previousShortName}: ${previousRegisterMode} = RegisterDefinition::split_array(${previousOffset}, ${previousStep}, ${previousMin}, ${previousMax}, ${offset}, ${step}, ${min}, ${max});
EOF
}

write_array()
{
	local longName="$1"
	local shortName="$2"
	local registerMode="$3"
	local offset="$4"
	local step="$5"
	local min="$6"
	local max="$7"

	cat <<EOF

/// ${longName}.
pub const ${shortName}: ${registerMode} = RegisterDefinition::array(${offset}, ${step}, ${min}, ${max});
EOF
}

write_singleton()
{
	local longName="$1"
	local shortName="$2"
	local registerMode="$3"
	local offset="$4"

	cat <<EOF

/// ${longName}.
pub const ${shortName}: ${registerMode} = RegisterDefinition::singleton(${offset});
EOF
}

write_array_or_singleton()
{
	local longName="$1"
	local shortName="$2"
	local registerMode="$3"
	local offset="$4"
	local step="$5"
	local min="$6"
	local max="$7"

	if [ -n "$step" ]; then
		write_array "$longName" "$shortName" "$registerMode" "$offset" "$step" "$min" "$max"
	else
		write_singleton "$longName" "$shortName" "$registerMode" "$offset"
	fi
}

parse_internal()
{
	local file="$1"

	local shortName
	local offset
	local mode
	local longName
	local step
	local min
	local max

	local previousShortName
	local previousOffset
	local previousLongName
	local previousStep
	local previousMin
	local previousMax
	local previousRegisterMode
	local waitForNextRowBeforeWriting
	reset_previous

	local indexing
	while IFS=' ' read -r shortName offset indexing mode longName
	do
		local registerMode
		register_mode "$mode"

		parse_indexing "$indexing" "$shortName" "$file"

		if $waitForNextRowBeforeWriting; then
			store_current_as_previous
			waitForNextRow=false
		elif [ "$previousShortName" = "$shortName" ]; then
			if [ "$previousLongName" != "$longName" ]; then
				printf 'Invalid long name match with previous %s in %s\n' "$shortName" "$file"
				exit 1
			fi
			if [ "$previousRegisterMode" != "$registerMode" ]; then
				printf 'Invalid register mode match with previous %s in %s\n' "$shortName" "$file"
				exit 1
			fi

			write_split_array
			reset_previous
		else
			write_array_or_singleton "$previousLongName" "$previousShortName" "$previousRegisterMode" "$previousOffset" "$previousStep" "$previousMin" "$previousMax"
			store_current_as_previous
		fi
	done <"$file"
}

parse()
{
	local folderPath="$1"

	{
		cat <<EOF
// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.

EOF
	parse_internal "$folderPath"/registers.snabb
	} >"$folderPath"/registers.rs
}

process_folder()
{
	local relativePath="$1"

	if [ -f "$relativePath"/registers.snabb ]; then
		parse "$relativePath"
	fi

	local folder
	set +f
	for folder in "$relativePath"/*
	do
		set -f
		if [ ! -e "$folder" ]; then
			continue
		fi
		if [ -L "$folder" ]; then
			continue
		fi
		if [ ! -d "$folder" ]; then
			continue
		fi

		process_folder "$folder"
	done
	set -f
}

main()
{
	process_folder '.'
}

main "$@"
