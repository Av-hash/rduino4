use core;

#[derive(Clone,Copy)]
pub enum PortName {
    C
}

#[repr(C,packed)]
pub struct Port {
    pcr: [u32;32],
//    pcr1:u32, 
//    pcr2:u32,// Complete the struct below. See section 11.1.4 of the manual. Note it has continous memory representation of multiple ports but struct should only account for port C i.e. all registers beginning with PORTC_.
//    pcr3:u32,
//    pcr4:u32,
//    pcr5:u32,
//    pcr6:u32,
//    pcr7:u32,
//    pcr8:u32,
//    pcr9:u32,
//    pcr10:u32,
//    pcr11:u32,
//    pcr12:u32,
//    pcr13:u32,
//    pcr14:u32,
//    pcr15:u32,
//    pcr16:u32,
//    pcr17:u32,
//    pcr18:u32,
//    pcr19:u32,
//    pcr20:u32,
//    pcr21:u32,
//    pcr22:u32,
//    pcr23:u32,
//    pcr24:u32,
//    pcr25:u32,
//    pcr26:u32,
//    pcr27:u32,
//    pcr28:u32,
//    pcr29:u32,
//    pcr30:u32,
//    pcr31:u32,
   

}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
       match name{
           PortName::C=>{
            &mut *(0x40052000 as *mut Port)
           }
       } 
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
       (*self).pcr[p]=((*self).pcr[p] & 0b11111111111111111111110001111111) |(mode & 0b00000000000000000000001110000000); // Given the pin mode as a 32 bit value set the register bytes to the same value for the corresponding pin. See the MUX(10-8) bits in section 11.14.1. We need to set only those bits. Again think of appropriate operations using AND,OR,XOR etc.. There are only 8 possible pin models so mode = 0 to 7. Reject if different.
    }
}
pub struct Pin {
    port: *mut Port,
    pin: usize
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        let pin:Pin;
        pin.pin=p;
        pin.port=&mut self;
    }
}
#[repr(C,packed)]
struct GpioBitBand {
    PDOR:u32,
    PSOR:u32,
    PCOR:u32,
    PTOR:u32,
    PDIR:u32,
    PDDR:u32,
}


pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            0x4004B000 => return PortName::C,
            _ =>(), // Return PortName::C if the address matches the starting address of port C as specified in section 11.1.4. Reject if address is wrong and return error.
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port=self.port;
            let reg=port.p[self.pin];
            port.p[self.pin]=(reg & 0b0001111111) | (0b0010000000);
            let gpio:Gpio;// Set pin mode to 1 to enable gpio mode (section 11.14.1 MUX bits).
           gpio=Gpio::new(Port::name(self.port),self.pin) ;
           gpio;// Consume the pin into a gpio struct i.e. instantitate a gpio struct using the new function below.
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio:*mut GpioBitBand = match port {
            PortName::C => (0x43FE1000 as *mut GpioBitband)
        };
        let GPiO:Gpio;
        GPiO.gpio=gpio;
        GPiO.pin=pin;
        GPiO
        // Initialize and return a gpio struct.
    }

    pub fn output(&mut self) {
        unsafe {
           let gpio:*mut GpioBitBand=self.gpio;
           (*gpio).PDDR=1>>self.pin;
            //  WRITE THE  XX register of GPIO to 1 to enable this pin as output type.
            // See section 49.2 of the teensy manual to find out what is XX.
        }
    }

    pub fn high(&mut self) {
        unsafe {
            let gpio:*mut GpioBitBand=self.gpio;
           (*gpio).PDOR=1<<self.pin;
           //  WRITE THE  XX register of GPIO to 1 to set this pin as high.
           // See section 49.2 of the teensy manual to find out what is XX. Please not that it is not PDOR, since PDOR is never directly written.
        }
    }
    pub fn high(&mut self) {
        unsafe {
            let gpio:*mut GpioBitBand=self.gpio;
           (*gpio).PDOR=0<<self.pin;
        }
    }
}