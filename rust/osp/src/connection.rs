// SPDX-FileCopyrightText: 2023 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later

use std::{
    io::{self, BufReader, Write},
    os::unix::net::UnixStream,
    path::Path,
};

use crate::{
    commands::Error,
    response::{self, Response},
    ScanCommand, ScanID,
};

/// Sends a command to the unix socket and returns the response
pub fn send_command<T: AsRef<Path>>(address: T, cmd: ScanCommand) -> Result<Response, Error> {
    let mut socket = UnixStream::connect(address)?;
    let cmd = cmd.try_to_xml()?;
    socket.write_all(&cmd)?;
    let reader: BufReader<_> = BufReader::new(socket);

    quick_xml::de::from_reader(reader).map_err(|e| e.into())
}

/// Returns the scan information from OSPD
pub fn get_scan<T: AsRef<Path>>(address: T, scan_id: &ScanID) -> Result<response::Scan, Error> {
    let cmd = ScanCommand::Get(scan_id);
    let response = send_command(address, cmd)?;
    match response {
        Response::GetScans {
            status: _,
            scan: Some(scan),
        } => Ok(scan),
        _ => Err(Error::InvalidResponse(response.into())),
    }
}

/// Returns the scan information from OSPD and deletes the results from it
pub fn get_delete_scan_results<T: AsRef<Path>>(
    address: T,
    scan_id: &str,
) -> Result<response::Scan, Error> {
    let cmd = ScanCommand::GetDelete(scan_id);
    let response = send_command(address, cmd)?;
    match response {
        Response::GetScans {
            status: _,
            scan: Some(scan),
        } => Ok(scan),
        _ => Err(Error::InvalidResponse(response.into())),
    }
}

/// Starts a scan
pub fn start_scan<T: AsRef<Path>>(address: T, scan: &models::Scan) -> Result<ScanID, Error> {
    let cmd = ScanCommand::Start(scan);
    let response = send_command(address, cmd)?;
    match response {
        Response::StartScan {
            status: _,
            id: Some(id),
        } => Ok(id),
        _ => Err(Error::InvalidResponse(response.into())),
    }
}

/// Stops a scan
pub fn stop_scan<T: AsRef<Path>>(address: T, scan_id: &ScanID) -> Result<(), Error> {
    let cmd = ScanCommand::Stop(scan_id);
    let response = send_command(address, cmd)?;
    match response {
        Response::StopScan { status: _ } => Ok(()),
        _ => Err(Error::InvalidResponse(response.into())),
    }
}

/// Deletes a scan
pub fn delete_scan<T: AsRef<Path>>(address: T, scan_id: &ScanID) -> Result<(), Error> {
    let cmd = ScanCommand::Delete(scan_id);
    let response = send_command(address, cmd)?;
    match response {
        Response::DeleteScan { status: _ } => Ok(()),
        _ => Err(Error::InvalidResponse(response.into())),
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Socket(err.kind())
    }
}
