//! provides a basic session watcher which wraps [I2pListener::accept] ensuring that
//! any errors which result in the session being terminated, such as clients improperly disconnecting
//! or other network/transport level issues are handled gracefully. 
//! 
//! any calls to accept which result in an error will cause the existing session and i2plistener to be dropped,
//! before they are recreated and an error is returned information the caller to try the operation again
//! 


use std::net::Shutdown;


use crate::{sam::{StreamConnect, SessionStyle, nickname}, net::{I2pSocketAddr, I2pListener}, Session, sam_options::SAMOptions, Error, ErrorKind};
use log::{info, warn, error};

/// SamSessionWatcher provides the ability to gracefully handle
/// runtime errors by restarting the sam session, and recreating the listener
/// any time errors are detected. 
/// 
/// note: should implement better detection of which errors cause us 
///       to recreate the connection
pub struct SamSessionWatcher {
    opts: SAMOptions,
    session: Session,
    destination: String,
    sam_endpoint: String,
    session_style: SessionStyle,
    pub listener: I2pListener,
}

impl SamSessionWatcher {
    pub fn new(
        sam_endpoint: &str,
        destination: &str,
        session_style: SessionStyle,
        opts: SAMOptions,
    ) -> Result<Box<SamSessionWatcher>, Error> {
        let (session, listener) = SamSessionWatcher::__recreate(
            sam_endpoint,
            destination,
            &nickname(),
            session_style.clone(),
            opts.clone()
        )?;
        Ok(Box::new(SamSessionWatcher {
            opts,
            session,
            listener,
            session_style,
            destination: destination.to_string(),
            sam_endpoint: sam_endpoint.to_string(),
        }))
    }
    pub fn accept(self: &mut Box<Self>) -> Result<(StreamConnect, I2pSocketAddr), Error> {
        match self.listener.forward.accept() {
            Ok(res) => Ok(res),
            Err(err) => {
                error!("accept encountered error, recreating stream: {:#?}", err);
                {
                    drop(&mut self.listener);
                    self.session.sam.conn.shutdown(Shutdown::Both)?;
                    drop(&mut self.session);
                }
                self.recreate()?;
                Err(ErrorKind::SessionRecreated.into())
            }
        }
    }
    fn recreate(self: &mut Box<Self>) -> Result<(), Error> {
        let (session, listener) = SamSessionWatcher::__recreate(
            &self.sam_endpoint,
            &self.destination,
            &nickname(),
            self.session_style.clone(),
            self.opts.clone()
        )?;
        self.session = session;
        self.listener = listener;
        Ok(())
    }
    fn __recreate(
        sam_endpoint: &str,
        destination: &str,
        nickname: &str,
        session_style: SessionStyle,
        opts: SAMOptions,
    ) -> Result<(Session, I2pListener), Error> {
        let session = Session::create(
            sam_endpoint,
            destination,
            nickname,
            session_style,
            opts.clone(), 
        )?; 
        let listener = I2pListener::bind_with_session(&session)?; 
        Ok((session, listener))
    }
}