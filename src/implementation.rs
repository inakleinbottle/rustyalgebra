


pub(crate) const fn const_logn<const N: u32>(n: u32) -> u32
{
    if n < 2 {
        return 0;
    } else {
        return const_logn::<N>(n / 2) + 1;
    }
}

pub(crate) const fn make_mask(n_bits: u32, shift: u32) -> u64
{
    ((1u64 << n_bits) - 1) << shift
}