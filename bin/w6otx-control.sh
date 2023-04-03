#!/bin/bash

set -o nounset
set -o errexit

SNMP_AGENT=apc-rpdu
SNMP_COMMON=(-m +PowerNet-MIB -v1)
SNMP_PUBLIC=("${SNMP_COMMON[@]}" -c public "${SNMP_AGENT}")
SNMP_PRIVATE=("${SNMP_COMMON[@]}" -c private "${SNMP_AGENT}")
    
RPDU2=".iso.org.dod.internet.private.enterprises.apc.products.hardware.rPDU2"
SWITCHED_OUTLET="${RPDU2}.rPDU2Outlet.rPDU2OutletSwitched"
STATUS_N="${SWITCHED_OUTLET}.rPDU2OutletSwitchedStatusTable.rPDU2OutletSwitchedStatusEntry.rPDU2OutletSwitchedStatusState"
COMMAND_N="${SWITCHED_OUTLET}.rPDU2OutletSwitchedControlTable.rPDU2OutletSwitchedControlEntry.rPDU2OutletSwitchedControlCommand"

get_power_state() {
    local plug_number=$1

    snmpget "${SNMP_PUBLIC[@]}" -Os "${STATUS_N}.${plug_number}"
}

set_power_state() {
    local plug_number=$1
    local state=$2

    snmpset "${SNMP_PRIVATE[@]}" "${COMMAND_N}.${plug_number}" = "${state}"
}

set_power_on() {
    local plug_number=$1
    local immediate_on=1
    set_power_state "${plug_number}" "${immediate_on}"
}

set_power_off() {
    local plug_number=$1
    local immediate_off=2
    set_power_state "${plug_number}" "${immediate_off}"
}

die() {
    local rc=$1
    shift
    echo "$0:" "$@" >&2
    exit "${rc}"
}

usage() {
    echo "Usage: $0 {get_status|turn_on|turn_off} {plug_name}" 2>&1
    exit 1
}


readonly -A PLUG_MAP=(
    [Battery_Charger]=1
    [Unused2]=2
    [Unused3]=3
    [Unused4]=4
    [Unused5]=5
    [UHF_DMR]=6
    [900MHz]=7
    [VHF_DMR]=8
    )

if (( $# != 2 )); then
    usage
fi

action="$1"
plug_name="$2"

if [[ ${PLUG_MAP[$plug_name]:-unset} == "unset" ]]; then
    echo "Known plug names:" >&2
    for name in "${!PLUG_MAP[@]}"; do
	echo "  ${name}" >&2
    done
    die 2 "Error: unknown plug name \"${plug_name}\""
fi

plug_number="${PLUG_MAP[${plug_name}]}"

case "${action}" in
    get_status)
	echo "Status of ${plug_name} (${plug_number})"
	get_power_state "${plug_number}"
	;;
    turn_off)
	echo "Turning off ${plug_name} (${plug_number})"
	set_power_off "${plug_number}"
	sleep 2
	get_power_state "${plug_number}"
	;;
    turn_on)
	echo "Turning on ${plug_name} (${plug_number})"
	set_power_on "${plug_number}"
	sleep 2
	get_power_state "${plug_number}"
	;;
    *)
	usage
	;;
esac


# Local Variables:
# compile-command: "shellcheck --format=gcc w6otx-control.sh"
# End:
