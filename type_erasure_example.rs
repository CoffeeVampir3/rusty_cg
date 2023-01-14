trait A {}
trait B {
  type X: A;
  fn test(&self) -> &Self::X;
}
trait C {
  fn test(&self) -> &dyn A;
}

impl A for i32 {}
impl B for (i32,) {
  type X = i32;

  fn test(&self) -> &i32 {
    &self.0
  }
}

impl<T> C for T
where
  T: B,
  <T as B>::X: A
{
  fn test(&self) -> &dyn A {
    <T as B>::test(self)
  }
}