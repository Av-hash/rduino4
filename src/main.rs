#![feature(stdsimd)]
#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::empty_loop)]

mod port;
mod sim;
mod watchdog;
use core::u8;
pub extern "C" fn main() {
    let watdog=Watchdog::new();
   (watdog).disable();
   let sim=Sim::new();
   sim.enable_clock(&mut self,PortC);
   let port=Port::new(C);
   let pin=port.pin(&mut self,2);
   let gpio:Gpio=pin.make_gpio(self);
   
    loop {
       
    }
}

extern "C" {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];
const FSEC:u8=0xF9;
const FOPT:u8=0xDE;
#[link_section = ".flashconfig"]
#[no_mangle]

pub static _FLASHCONFIG:[u8;16]=[
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, FSEC, FOPT, 0xFF, 0xFF,
 ];

#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {
        
    }
}
