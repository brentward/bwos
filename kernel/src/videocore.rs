use pi::videocore_mailbox::VideoCoreMailbox;

pub struct VideoCore {
    inner: Option<VideoCoreMailbox>,
}

impl VideoCore {
    /// Creates a new instance of `VideoCore`.
    const fn new() -> VideoCore {
        VideoCore { inner: None }
    }

    /// Initializes the console if it's not already initialized.
    #[inline]
    fn initialize(&mut self) {
        match self.inner {
            None => self.inner = Some(VideoCoreMailbox::new()),
            _ => (),
        }
    }

    /// Returns a mutable borrow to the inner `MiniUart`, initializing it as
    /// needed.
    fn inner(&mut self) -> &mut VideoCoreMailbox {
        match self.inner {
            Some(ref mut mailbox) => mailbox,
            _ => {
                self.initialize();
                self.inner()
            }
        }
    }
}
