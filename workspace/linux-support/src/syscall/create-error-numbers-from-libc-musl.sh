#!/usr/bin/env sh
# This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


set -e
set -u
set -f

exit_error()
{
	printf '%s\n' "$1" 1>&2
	exit 1
}

print_value()
{
	local name="$1"
	local value="$2"

	printf '%s\t%s\t%s\n' "$name" "$value" "$architecture" >>"$unsorted_file_path"
}

print_unknown_value()
{
	local name
	case ${#next_error_number} in

        1)
        	name="____$next_error_number"
        ;;

      	2)
        	name="___$next_error_number"
        ;;

      	3)
        	name="__$next_error_number"
        ;;

      	4)
        	name="_$next_error_number"
        ;;

    	*)
    		printf '%s\n' "Error: Length was ${#next_error_number}" 1>&2
    		exit 1
    	;;

    esac

	print_value "$name" "$next_error_number"
}

parse_header_file()
{
	local architecture="$1"

	local header_file_path="$musl_folder_path/arch/$architecture/bits/errno.h"
	local unsorted_file_path=unsorted."$architecture"
	local alias_file_path=alias."$architecture"
	local sorted_file_path=sorted."$architecture"

	touch "$unsorted_file_path"
	touch "$alias_file_path"

	local next_error_number=1

	local hash_define
	local name
	local error_number
	while IFS=' ' read -r hash_define name error_number
	do
		case "$error_number" in

			# Not a number but an alias.
            ''|*[!0-9]*)
				local rust_cfg_expression
				if [ "$architecture" = "generic" ]; then
					rust_cfg_expression='#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))]'
				else
					rust_cfg_expression="#[cfg(target_arch = \"$architecture\")]"
				fi
				printf '\t#[allow(missing_docs)]\n\t%s\n\tpub const %s: Self = SystemCallNumber::%s;\n\n' "$rust_cfg_expression" "$name" "$error_number" >>"$alias_file_path"
            ;;

            *)
            	while [ $next_error_number != $error_number ]
				do
					print_unknown_value
					next_error_number=$((next_error_number + 1))
				done
				print_value "$name" "$error_number"
				next_error_number=$((error_number + 1))
            ;;

        esac
	done <"$header_file_path"

	while [ $next_error_number != 4096 ]
	do
		print_unknown_value
		next_error_number=$((next_error_number + 1))
	done

	sort -b "$unsorted_file_path" >"$sorted_file_path"
}

generate_rust_code_enum_member()
{
	local name="$1"
	local code="$2"
	shift 2

	local rust_cfg_expression
	local initial_count=$#

	case $initial_count in

		0)
			exit_error "Missing architecture"
		;;

		1)
			rust_cfg_expression="target_arch = \"$1\""
		;;

		*)
			while [ $# -ne 0 ]
			do
				if [ $# -ne $initial_count ]; then
					printf ', '
				fi
				printf 'target_arch = "%s"' "$1"
				shift 1
			done >rust_cfg_expression
			rust_cfg_expression="any($(cat rust_cfg_expression))"
		;;

	esac

	printf '\t#[allow(missing_docs)]\n\t#[cfg(%s)]\n\t%s = %s,\n\n' "$rust_cfg_expression" "$name" "$code"
}

generate_rust_code_snippet()
{
	local all_sorted_file_path=all-sorted
	join -1 1 -2 1 -a 1 -a 2 sorted.generic sorted.mips64 | join -1 1 -2 1 -a 1 -a 2 - sorted.powerpc64 >"$all_sorted_file_path"

	local name
	local code0
	local architecture0
	local code1
	local architecture1
	local code2
	local architecture2
	while IFS=' ' read -r name code0 architecture0 code1 architecture1 code2 architecture2
	do
		# NOTE: the value 'generic' can only be present in $architecture0

		# Only supported on architecture0
		if [ -z "${architecture1}${architecture2}" ]; then
			if [ "$architecture0" = "generic" ]; then
				generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64
			else
				generate_rust_code_enum_member "$name" "$code0" "$architecture0"
			fi
		# Only supported on architecture0 and architecture1
		elif [ -z "$architecture2" ]; then
			if [ $code0 -eq $code1 ]; then
				if [ "$architecture0" = "generic" ]; then
					generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64 "$architecture1"
				else
					generate_rust_code_enum_member "$name" "$code0" "$architecture0" "$architecture1"
				fi
			else
				if [ "$architecture0" = "generic" ]; then
					generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64
				else
					generate_rust_code_enum_member "$name" "$code0" "$architecture0"
				fi
				generate_rust_code_enum_member "$name" "$code1" "$architecture1"
			fi
		# Supported on all (architecture0, architecture1 and architecture3)
		else
			if [ $code0 -eq $code1 ]; then
				# All architectures have same value: code0 == code1 == code2
				if [ $code1 -eq $code2 ]; then
					generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64 "$architecture1" "$architecture2"
				# Only architectures 0 and 1 have the same value: code0 == code1 != code2
				else
					if [ "$architecture0" = "generic" ]; then
						generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64 "$architecture1"
					else
						generate_rust_code_enum_member "$name" "$code0" "$architecture1"
					fi
					generate_rust_code_enum_member "$name" "$code2" "$architecture2"
				fi
			else
				#  Only architectures 1 and 2 have the same value: code0 != code1 == code2
				if [ $code1 -eq $code2 ]; then
					if [ "$architecture0" = "generic" ]; then
						generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64
					else
						generate_rust_code_enum_member "$name" "$code0" "$architecture0"
					fi
					generate_rust_code_enum_member "$name" "$code1" "$architecture1" "$architecture2"
				# code0 != code1 != code2
				else
					if [ "$architecture0" = "generic" ]; then
						generate_rust_code_enum_member "$name" "$code0" aarch64 riscv64 s390x x86_64
					else
						exit_error "Error: architecture0 was not generic but $architecture0"
					fi
					generate_rust_code_enum_member "$name" "$code1" "$architecture1"
					generate_rust_code_enum_member "$name" "$code2" "$architecture2"
				fi
			fi
		fi
	done <"$all_sorted_file_path" >"code.rs"
}

main()
{
	local musl_folder_path="$1"
	cd "$musl_folder_path" 1>/dev/null
		musl_folder_path="$(pwd)"
	cd - 1>/dev/null

	local temp_folder_path='./create-error-numbers-from-libc-musl.tmp'
	rm -rf "$temp_folder_path"
	mkdir -m 0700 -p "$temp_folder_path"
	cd "$temp_folder_path" 1>/dev/null 2>/dev/null

	# NOTE: If extending number of architectures, hardcoded logic in generate_rust_code_snippet needs to change
	local architecture
	for architecture in 'generic' 'mips64' 'powerpc64'
	do
		parse_header_file "$architecture"
	done

	generate_rust_code_snippet

	exit 99


	sort create-error-numbers-from-libc-musl.known.tmp | while IFS=$'\t' read -r name value architecture
	do
		local rust_cfg_expression
		if [ "$architecture" = "generic" ]; then
			rust_cfg_expression='#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))]'
		else
			rust_cfg_expression="#[cfg(target_arch = \"$architecture\")]"
		fi
		printf '\t#[allow(missing_docs)]\n\t%s\n\t%s = %s,\n\n' "$rust_cfg_expression" "$name" "$value" >>create-error-numbers-from-libc-musl.known
	done
}

main "$@"
