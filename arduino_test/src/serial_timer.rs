use core::cell;

const PRESCALER: u32 = 8;
const TIMER_COUNTS: u32 = 17;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));


//57600
pub fn serial_init(tc: arduino_hal::pac::TC2) {
    
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc.tccr2a.write(|w| w.wgm2().ctc());
    tc.ocr2a.write(|w| w.bits(TIMER_COUNTS as u8));
    tc.tccr2b.write(|w| w.cs2().prescale_8());
    tc.timsk2.write(|w| w.ocie2a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + 1);
    })
}