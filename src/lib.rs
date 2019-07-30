
use no_std_compat::prelude::v1::*;

use stm32f1xx_hal::gpio::{Input, PullUp, Output, PushPull};
use embedded_hal::digital::{OutputPin, InputPin};
use smallbitvec::{SmallBitVec};

/// A keyboard matrix (keypad matrix) that's 
/// based on a boxed trait objects
/// unlike Keypad and the other one TODO
pub struct BoxedKeyMatrix {
    state: SmallBitVec,
    rows: Vec<Output<PushPull>>,
    columns: Vec<Input<PullUp>>,
}

impl BoxedKeyMatrix {
    pub fn new(
    rows: Vec<Output<PushPull>>,
    columns: Vec<Input<PullUp>>
    ) -> BoxedKeyMatrix
{
        for p in columns.iter() {
            p.set_high(); // turn on pull up
        }
        let mut state = SmallBitVec::with_capacity(rows.len() * columns.len());
        return BoxedKeyMatrix{
            rows,
            columns,
            state
        }
    }

    pub fn read_matrx(&mut self) -> &SmallBitVec {
        self.state.clear(); //this does not deallocate
        //establish base state
        for r in self.rows.iter_mut() {
            r.set_high();
        }
        for r in self.rows.iter_mut() {
            //enable row
            r.set_low();
            for c in self.columns.iter_mut() {
                //read columns
                self.state.push(c.is_low());
            }
            //disable row
            r.set_high();
        }
    return &self.state;
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
