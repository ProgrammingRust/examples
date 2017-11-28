pub struct Session;

pub fn connect() -> Session {
    Session
}

impl Session {
    /// Upload all local terrariums to the online gallery.
    ///
    /// ```no_run
    /// let mut session = fern_sim::connect();
    /// session.upload_all();
    /// ```
    pub fn upload_all(&mut self) {
        unimplemented!();
    }
}
