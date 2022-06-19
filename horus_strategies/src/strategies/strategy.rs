pub trait Strategy {
    fn run_hot_path(&self);
    fn get_name() -> &'static str;
}