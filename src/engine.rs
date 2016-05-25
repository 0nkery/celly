trait Engine {
    fn set_grid<T: Grid>(&self, grid: T);
    fn run(&self);
}