pub fn to_sized_unchecked<T, const N: usize>(slice: &[T]) -> &[T; N] {
    assert_eq!(slice.len(), N);
    unsafe { &*slice.as_ptr().cast() }
}