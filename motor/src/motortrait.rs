pub trait Motor {
    fn init()->Self;
    fn goto(&mut self, pos: i64) -> Result<(), ()>;
    fn update(&mut self);
    fn stop(& mut self);
    fn set_zero(&mut self);
    fn sync(&mut self);
}
