use crate::*;

/// Provides access to the UART interface.
///
/// See: [`Uart::read()`] and [`Uart::write()`].
pub struct Uart<'a> {
    sim: &'a mut AvrSimulator,
    id: char,
}

impl<'a> Uart<'a> {
    pub(crate) fn new(sim: &'a mut AvrSimulator, id: char) -> Self {
        Self { sim, id }
    }

    /// Retrieves a value from AVR.
    ///
    /// See: [`Readable`].
    /// See also: [`Self::try_read_byte()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::*;
    /// # fn avr() -> AvrTester { panic!() }
    /// #
    /// let mut avr = avr();
    ///
    /// // Retrieves a single byte:
    /// // (when the input buffer is empty, panics.)
    /// assert_eq!(72, avr.uart0().read::<u8>());
    ///
    /// // Retrieves the entire buffer:
    /// // (when it's empty, returns an empty vector.)
    /// assert_eq!(vec![72, 101, 108, 108, 111], avr.uart0().read::<Vec<u8>>());
    ///
    /// // Retrieves `n` bytes from the buffer:
    /// // (when there's not enough bytes, panics.)
    /// assert_eq!([72, 101, 108, 108, 111], avr.uart0().read::<[u8; 5]>());
    ///
    /// // Retrieves the entire input buffer and converts it into string:
    /// // (when it's empty, returns an empty string.)
    /// assert_eq!("Hello", avr.uart0().read::<String>());
    /// ```
    pub fn read<T>(&mut self) -> T
    where
        T: Readable,
    {
        T::read(self)
    }

    /// Retrieves a single byte from AVR.
    ///
    /// As compared to [`Self::read()`], when the buffer is empty, this function
    /// returns `None` instead of panicking.
    ///
    /// When this function returns `None`, it will continue to return `None` at
    /// least up until the next call to [`AvrTester::run()`], since that's when
    /// AvrTester "pulls" bytes from the simulated AVR.
    ///
    /// See also: [`Self::read()`].
    pub fn try_read_byte(&mut self) -> Option<u8> {
        self.sim.read_uart(self.id)
    }

    /// Transmits a value to AVR.
    ///
    /// See: [`Writable`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use avr_tester::*;
    /// # fn avr() -> AvrTester { panic!() }
    /// #
    /// let mut avr = avr();
    ///
    /// // Transmits a single byte:
    /// avr.uart0().write(123);
    ///
    /// // Transmits many bytes:
    /// avr.uart0().write([10, 20, 30]);
    ///
    /// // Transmits a string:
    /// avr.uart0().write("Hello!");
    ///
    /// // Strings are transmitted as a series of their bytes, so the above is
    /// // equivalent to:
    /// avr.uart0().write([72, 101, 108, 108, 111, 33]);
    /// //                 H   e    l    l    o    !
    /// ```
    pub fn write<T>(&mut self, value: T)
    where
        T: Writable,
    {
        value.write(self);
    }
}

impl Reader for Uart<'_> {
    fn read_byte(&mut self) -> u8 {
        self.try_read_byte().expect(
            "UART's buffer is empty - got no more bytes to read; if you're \
             receiving a large buffer, try running the simulator for a bit \
             longer so that the simulated AVR has more time to respond",
        )
    }

    fn try_read_byte(&mut self) -> Option<u8> {
        self.sim.read_uart(self.id)
    }
}

impl Writer for Uart<'_> {
    fn write_byte(&mut self, value: u8) {
        self.sim.write_uart(self.id, value);
    }
}
