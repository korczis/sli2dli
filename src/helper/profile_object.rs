use time::PreciseTime;

pub struct ProfileObject {
    pub start: PreciseTime
}

impl ProfileObject {
    pub fn new() -> ProfileObject{
        ProfileObject {
            start: PreciseTime::now()
        }
    }
}

impl Drop for ProfileObject {
    fn drop(&mut self) {
        let diff = self.start.to(PreciseTime::now());
        let elapsed_secs = diff.num_nanoseconds().unwrap() as f64 * 1e-9;

        debug!("Drop() - {:?}", elapsed_secs);
    }
}
