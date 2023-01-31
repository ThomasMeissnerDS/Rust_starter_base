pub struct Stepper {
    curr: u32,
    step: u32,
    max: u32,
}

impl Iterator for Stepper {  /*Impl gives functionality to an object*/
    type Item = u32; // type is needed here
    // needs to update itself
    fn next(&mut self) -> Option<u32> { // Optional as we will return done when Iterator is exhausted
        if self.curr >= self.max {
            return None
        }
        let res = self.curr;
        self.curr += self.step;
        return Some(res)
    }
}

fn main() {
    let mut st = Stepper{curr: 35, step: 2, max: 96};
    loop {
        match st.next() {
            Some(v) => println!("We are at {}.", v), // could also be replaced by st.curr in the print
            None => break
        }
    }

    println!("All done");
}