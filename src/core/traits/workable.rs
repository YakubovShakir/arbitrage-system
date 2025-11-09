pub trait Workable {
    fn id(&self) -> usize;
    async fn run(&self) -> !;
}
