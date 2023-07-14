extern crate termios_sys;

pub use ::os::common::*;

pub use self::termios_sys::{
    termios,
    NCCS,

    // c_cc characters
    VWERASE,
    VREPRINT,
    VLNEXT,
    VDISCARD,

    // c_iflag bits
    IMAXBEL,

    // c_oflag bits
    TAB3,
    OXTABS,
    ONOEOT,

    // c_cflag bits
    CIGNORE,
    CRTSCTS,
    CRTS_IFLOW,
    CCTS_OFLOW,
    MDMBUF,

    // c_lflag bits
    ALTWERASE,
    EXTPROC,
    FLUSHO,
    NOKERNINFO,
    PENDIN,

    // baud rates
    B7200,
    B14400,
    B28800,
    B57600,
    B76800,
    B115200,
    B230400,
    EXTA,
    EXTB,

    TCSASOFT,
};
