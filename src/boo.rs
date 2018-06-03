use std::convert::AsRef;
use std::ops::Deref;

///
/// Borrowed or Owned
/// Very similar to `std::borrow::Cow` but without requiring `ToOwned` (and therefore, `Clone`)
///
pub enum Boo<'a, B: 'a> {
    Borrowed(&'a B),
    Owned(B),
}

impl<'a, B: 'a> AsRef<B> for Boo<'a, B> {
    fn as_ref(&self) -> &B {
        match self {
            &Boo::Borrowed(b) => b,
            &Boo::Owned(ref o) => o,
        }
    }
}

impl<'a, B: 'a> Deref for Boo<'a, B> {
    type Target = B;
    fn deref(&self) -> &B {
        self.as_ref()
    }
}
