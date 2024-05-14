/// Trait for converting a value from its corresponding [`joltc-sys`] type.
pub trait FromJolt {
    type Jolt;

    fn from_jolt(value: Self::Jolt) -> Self;
}

/// Trait for converting a value into its corresponding [`joltc-sys`] type.
pub trait IntoJolt {
    type Jolt;

    fn into_jolt(self) -> Self::Jolt;
}

/// Convenience trait for [`FromJolt`].
pub trait IntoRolt<Rolt> {
    fn into_rolt(self) -> Rolt;
}

impl<J, R> IntoRolt<R> for J
where
    R: FromJolt<Jolt = J>,
{
    fn into_rolt(self) -> R {
        R::from_jolt(self)
    }
}
