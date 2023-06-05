pub type S = Generator<290797, 50515093>;

#[derive(Debug)]
pub struct Generator<const INITAL: usize, const MOD: usize> {
    pub value: usize,
}

impl<const INITIAL: usize, const MOD: usize> Generator<INITIAL, MOD> {
    pub fn new() -> Self {
        Self { value: INITIAL }
    }
}

impl<const INITIAL: usize, const MOD: usize> Iterator for Generator<INITIAL, MOD> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = self.value * self.value % MOD;
        Some(self.value)
    }
}

