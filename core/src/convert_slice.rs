use num_traits::Num;

pub fn to_bytes<T: Num>(numbers: &[T]) -> &[u8] {
    let size = numbers.len() * std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(numbers.as_ptr() as *const _, size) }
}

pub fn from_bytes<T: Num>(bytes: &[u8]) -> &[T] {
    let size = bytes.len() / std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const _, size) }
}
