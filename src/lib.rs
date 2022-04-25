use std::{fmt::Debug, cmp::Ordering};
use std::hash::{Hash, Hasher};

pub struct UPair<T> {
    a: T,
    b: T,
}

impl<T> UPair<T>
where
    T: Ord,
{
    pub fn new(a: T, b: T) -> Self {
        let (f, s) = if a < b { (a, b) } else { (b, a) };
        Self { a: f, b: s }
    }

    pub fn first(&self) -> &T {
        &self.a
    }

    pub fn second(&self) -> &T {
        &self.b
    }
}

impl<T> PartialEq<UPair<T>> for UPair<T>
where T: PartialEq<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<T> Eq for UPair<T> where T: Eq {}

impl<T> Clone for UPair<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl<T> PartialOrd for UPair<T> where T: PartialOrd {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.a.partial_cmp(&rhs.a) {
            Some(Ordering::Equal) => self.b.partial_cmp(&rhs.b),
            v => v,
        }
    }
}

impl<T> Ord for UPair<T> where T: Ord {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self.a.cmp(&rhs.a) {
            Ordering::Equal => self.b.cmp(&rhs.b),
            v => v,
        }
    }
}

impl<T> Copy for UPair<T> where T: Copy {}

impl<T> Debug for UPair<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UPair")
            .field("first", &self.a)
            .field("second", &self.b)
            .finish()
    }
}

impl<T> Hash for UPair<T> where T: Hash {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.a.hash(hasher);
        self.b.hash(hasher);
    }
}

impl<T> From<(T, T)> for UPair<T> where T: Ord{
    fn from((a,b): (T, T)) -> Self {
        Self::new(a, b)
    }
}

impl<T> From<[T; 2]> for UPair<T> where T: Ord{
    fn from([a,b]: [T; 2]) -> Self {
        Self::new(a, b)
    }
}

impl<T> IntoIterator for UPair<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.a, self.b].into_iter()
    }
}
