use core;

#[derive(Clone,Copy)]
pub enum Clock {
    PortC,
}

#[repr(C,packed)]
pub struct Sim {
  sopt1:u32,
  sopt1cfg:u32,
  pad1:[u32;1023],
  sopt2:u32,
  pad2:u32,
  sopt4:u32,
  sopt5:u32,
  pad3:u32,
  sopt7:u32,
  pad4:u32,
  pad5:u32,
  sdid:u32,
  scgc1:u32,
  scgc2:u32,
  scgc3:u32,
  scgc4:u32,
  scgc5:u32,
  scgc6:u32,
  scgc7:u32,
  clkdiv1:u32,
  clkdiv2:u32,
  fcfg1:u32,
  fcfg2:u32,
  uidh:u32,
  uidmh:u32,
  uidml:u32,
  uidl:u32,
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
       & mut *(4004_7000 as  *mut Sim ) 
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                let mut ctrl=core::ptr::read_volatile(&mut Sim.scgc5);
                ctrl=ctrl | (0b00000000000000000000010000000000);
                core::ptr::write_volatile(&mut Sim.scgc5,ctrl);  
                }
            }
        }
    }
}