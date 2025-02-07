#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Axis(pub usize);

impl Axis {
    #[inline(always)]
    pub fn index(self) -> usize {
        self.0
    }
}
