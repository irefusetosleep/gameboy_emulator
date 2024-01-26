use std::os::raw::c_uchar;

struct Registers { // cpu registers
    a: c_uchar,
    b: c_uchar,
    c: c_uchar,
    d: c_uchar,
    e: c_uchar,
    f: c_uchar,
    h: c_uchar,
    l: c_uchar
}

struct Flags { // cpu flags
    z: c_uchar, // ZERO FLAG; set to 1 if current operation results in Zero, or two values match on a CMP operation
    n: c_uchar, // SUBTRACT FLAG; set to 1 if a subtraction was performed
    h: c_uchar, // HALF CARRY FLAG; set to 1 if a carry occured from the lower nibble in the last operation
    c: c_uchar, // CARRY FLAG; set to 1 if a carry occured in the last operation or if A is the smaller value on CP instruction
    halt: bool
}

