use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use snmp::{SnmpError, SyncSession, Value};

#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
pub enum OutletStatus {
    Off = 1,
    On = 2,
}

#[derive(Debug)]
#[repr(i64)]
pub enum OutletControlCommand {
    ImmediateOn = 1,
    ImmediateOff = 2,
    ImmediateReboot = 3,
    OutletUnknown = 4,
    DelayedOn = 5,
    DelayedOff = 6,
    DelayedReboot = 7,
    CancelPendingCommand = 8,
}

pub fn get_outlet_status(session: &mut SyncSession, n: u32) -> Result<OutletStatus, SnmpError> {
    let oid = make_outlet_status_oid(n);
    let result = session.get(&oid);
    match result {
        Ok(mut pdu) => match pdu.varbinds.next() {
            Some((_, Value::Integer(n))) => match OutletStatus::try_from(n) {
                Ok(outlet_status) => Ok(outlet_status),
                Err(..) => Err(SnmpError::ValueOutOfRange),
            },
            Some(_) | None => Err(SnmpError::ValueOutOfRange),
        },
        Err(error) => Err(error),
    }
}

pub fn control_outlet(
    session: &mut SyncSession,
    n: u32,
    command: OutletControlCommand,
) -> Result<(), SnmpError> {
    let oid = make_outlet_control_oid(n);
    let value = Value::Integer(command as i64);
    let result = session.set(&[(&oid, value)]);
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

fn make_outlet_status_oid(n: u32) -> [u32; 16] {
    // iso.org.dod.internet.private.enterprises.apc.products
    // .hardware.rPDU2.rPDU2Outlet.rPDU2OutletSwitched
    // .rPDU2OutletSwitchedStatusTable.rPDU2OutletSwitchedStatusEntry
    // .rPDU2OutletSwitchedStatusState.n
    [1, 3, 6, 1, 4, 1, 318, 1, 1, 26, 9, 2, 3, 1, 5, n]
}

fn make_outlet_control_oid(n: u32) -> [u32; 16] {
    // .iso.org.dod.internet.private.enterprises.apc.products
    // .hardware.rPDU2.rPDU2Outlet.rPDU2OutletSwitched
    // .rPDU2OutletSwitchedControlTable.rPDU2OutletSwitchedControlEntry
    // .rPDU2OutletSwitchedControlCommand.n
    [1, 3, 6, 1, 4, 1, 318, 1, 1, 26, 9, 2, 4, 1, 5, n]
}
