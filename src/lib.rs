#![doc = include_str!("../README.md")]

////////////////////////////////////////////////////////////////////////////////
// Timed Option
////////////////////////////////////////////////////////////////////////////////

/// See [module level documentation][crate]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TimedOption<T, Ttl> {
    value: Option<T>,
    ttl: Ttl,
}

impl<T, B> TimedOption<T, B>
where
    B: TtlBackend,
{
    /// Some value of type `T` with a ttl.
    #[inline]
    pub fn new(value: T, ttl: B::Duration) -> Self {
        TimedOption {
            value: Some(value),
            ttl: B::now().add(ttl),
        }
    }

    /// No value with a expired ttl
    #[inline]
    pub fn empty() -> Self {
        TimedOption {
            value: None,
            ttl: B::expired(),
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // consumption + as_refs
    ////////////////////////////////////////////////////////////////////////////

    /// Returns an `Option<T>`. If the value is some but expired a `None` is returned.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        match self.ttl.is_valid() {
            true => self.value,
            false => None,
        }
    }

    /// Returns an `Option<&T>`. If the value is some but expired a `None` is returned.
    #[inline]
    pub fn as_option(&self) -> Option<&T> {
        self.as_ref().into_option()
    }

    /// Returns an `TimedValue<T>`.
    #[inline]
    pub fn into_timed_value(self) -> TimedValue<T> {
        match (self.value, self.ttl.is_valid()) {
            (Some(value), true) => TimedValue::Valid(value),
            (Some(value), false) => TimedValue::Expired(value),
            (None, _) => TimedValue::None,
        }
    }

    /// Returns an `TimedValue<&T>`.
    #[inline]
    pub fn as_timed_value(&self) -> TimedValue<&T> {
        self.as_ref().into_timed_value()
    }

    /// Converts from `&TimedOption<T>` to `TimedOption<&T>`.
    #[inline]
    pub fn as_ref(&self) -> TimedOption<&T, B> {
        TimedOption {
            value: self.value.as_ref(),
            ttl: self.ttl.clone(),
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // mutations
    ////////////////////////////////////////////////////////////////////////////

    /// Expires the current ttl.
    #[inline]
    pub fn expire(&mut self) {
        self.ttl = B::expired();
    }

    /// Sets value to [`None`].
    #[inline]
    pub fn clear(&mut self) {
        self.value = None;
    }

    /// Takes the value out of the [`TimedOption`], returning an [`Option`] and
    /// leaving a [`None`] in its place.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        let value = self.value.take();
        match self.ttl.is_valid() {
            true => value,
            false => None,
        }
    }

    /// Takes the value out of the [`TimedOption`], Returning a [`TimedValue`]
    /// and leaving a [`None`] in its place.
    #[inline]
    pub fn take_timed_value(&mut self) -> TimedValue<T> {
        match (self.value.take(), self.ttl.is_valid()) {
            (Some(value), true) => TimedValue::Valid(value),
            (Some(value), false) => TimedValue::Expired(value),
            (None, _) => TimedValue::None,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // utility functions
    ////////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the timed-option is `Some` value and has not expired.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.value.is_some() & self.ttl.is_valid()
    }

    /// Returns `true` if the timed-option is `None` value or it has expired.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.value.is_none() | self.ttl.is_expired()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Timed Value
////////////////////////////////////////////////////////////////////////////////

/// An enum representing a value that is associated with a time validity status.
///
/// `TimedValue` can be used to indicate whether a value is valid, expired, or absent (none).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TimedValue<T> {
    /// value of type `T` that is considered valid
    Valid(T),
    /// value of type `T` that has expired and is no longer considered valid
    Expired(T),
    /// Indicates the absence of a value.
    None,
}

impl<T> TimedValue<T> {
    /// Returns `true` if the [`TimedValue`]` is `Valid`.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        match self {
            TimedValue::Valid(_) => true,
            TimedValue::Expired(_) => false,
            TimedValue::None => false,
        }
    }

    /// Returns `true` if the TimedValue is `Expired`.
    #[inline]
    pub const fn is_expired(&self) -> bool {
        match self {
            TimedValue::Valid(_) => false,
            TimedValue::Expired(_) => true,
            TimedValue::None => false,
        }
    }

    /// Returns `true` if the TimedValue is `None`.
    #[inline]
    pub const fn is_none(&self) -> bool {
        match self {
            TimedValue::Valid(_) => false,
            TimedValue::Expired(_) => false,
            TimedValue::None => true,
        }
    }

    /// Returns `true` if the TimedValue is `Valid` or `Expired`.
    #[inline]
    pub const fn has_value(&self) -> bool {
        match self {
            TimedValue::Valid(_) => true,
            TimedValue::Expired(_) => true,
            TimedValue::None => false,
        }
    }

    /// Converts from `&TimedValue<T>` to `TimedValue<&T>`.
    #[inline]
    pub const fn as_ref(&self) -> TimedValue<&T> {
        match *self {
            TimedValue::Valid(ref inner) => TimedValue::Valid(inner),
            TimedValue::Expired(ref inner) => TimedValue::Expired(inner),
            TimedValue::None => TimedValue::None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conversion
////////////////////////////////////////////////////////////////////////////////

impl<T, B> From<TimedOption<T, B>> for Option<T>
where
    B: TtlBackend,
{
    #[inline]
    fn from(value: TimedOption<T, B>) -> Self {
        value.into_option()
    }
}

impl<T, B> From<TimedOption<T, B>> for TimedValue<T>
where
    B: TtlBackend,
{
    #[inline]
    fn from(value: TimedOption<T, B>) -> Self {
        value.into_timed_value()
    }
}

////////////////////////////////////////////////////////////////////////////////
// TTL Backent
////////////////////////////////////////////////////////////////////////////////

pub trait TtlBackend: Clone {
    type Duration;

    fn now() -> Self;
    fn expired() -> Self;
    fn add(self, dt: Self::Duration) -> Self;
    fn is_valid(&self) -> bool;
    fn is_expired(&self) -> bool;
}

impl TtlBackend for std::time::Instant {
    type Duration = std::time::Duration;

    #[inline]
    fn now() -> Self {
        std::time::Instant::now()
    }

    #[inline]
    fn expired() -> Self {
        std::time::Instant::now()
    }

    #[inline]
    fn add(self, dt: Self::Duration) -> Self {
        self + dt
    }

    #[inline]
    fn is_valid(&self) -> bool {
        *self > std::time::Instant::now()
    }

    #[inline]
    fn is_expired(&self) -> bool {
        *self <= std::time::Instant::now()
    }
}

#[cfg(feature = "chrono")]
impl TtlBackend for chrono::DateTime<chrono::Utc> {
    type Duration = chrono::Duration;

    #[inline]
    fn now() -> Self {
        chrono::Utc::now()
    }

    #[inline]
    fn expired() -> Self {
        chrono::Utc::now()
    }

    #[inline]
    fn add(self, dt: Self::Duration) -> Self {
        self + dt
    }

    #[inline]
    fn is_valid(&self) -> bool {
        *self > chrono::Utc::now()
    }

    #[inline]
    fn is_expired(&self) -> bool {
        *self <= chrono::Utc::now()
    }
}
