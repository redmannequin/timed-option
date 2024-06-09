#![doc = include_str!("../README.md")]

use std::time::{Duration, Instant};

/// See [module level documentation][crate]
#[derive(Debug, Default, Copy, Clone)]
pub struct TimedOption<T> {
    inner: Option<(T, Instant)>,
}

impl<T> TimedOption<T> {
    /// Some value of type `T` with a ttl.
    #[inline]
    pub fn some(inner: T, ttl: Duration) -> Self {
        TimedOption {
            inner: Some((inner, Instant::now() + ttl)),
        }
    }

    /// None value.
    #[inline]
    pub const fn none() -> Self {
        TimedOption { inner: None }
    }

    /// Returns `true` if the timed-option is `Some` value and has not expired.
    #[inline]
    pub fn is_some(&self) -> bool {
        match self.inner {
            Some((_, ttl)) => ttl > Instant::now(),
            None => false,
        }
    }

    /// Returns `true` if the timed-option is `None` value or it has expired.
    #[inline]
    pub fn is_none(&self) -> bool {
        match self.inner {
            Some((_, ttl)) => ttl <= Instant::now(),
            None => true,
        }
    }

    /// Returns an `Option<T>`. If the value is some but expired a `None` is returned.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        match self.is_some() {
            true => unsafe { Some(self.inner.unwrap_unchecked().0) },
            false => None,
        }
    }

    /// Returns an `Option<&T>`. If the value is some but expired a `None` is returned.
    #[inline]
    pub fn as_option(&self) -> Option<&T> {
        match self.is_some() {
            true => unsafe { Some(&self.inner.as_ref().unwrap_unchecked().0) },
            false => None,
        }
    }

    #[inline]
    pub fn into_timed_value(self) -> TimedValue<T> {
        match self.inner {
            Some((inner, ttl)) => {
                if ttl > Instant::now() {
                    TimedValue::Valid(inner)
                } else {
                    TimedValue::Expired(inner)
                }
            }
            None => TimedValue::None,
        }
    }

    #[inline]
    pub fn as_timed_value(&self) -> TimedValue<&T> {
        match self.inner {
            Some((ref inner, ttl)) => {
                if ttl > Instant::now() {
                    TimedValue::Valid(inner)
                } else {
                    TimedValue::Expired(inner)
                }
            }
            None => TimedValue::None,
        }
    }

    /// Converts from `&TimedOption<T>` to `TimedOption<&T>`.
    #[inline]
    pub const fn as_ref(&self) -> TimedOption<&T> {
        match self.inner {
            Some((ref inner, ttl)) => TimedOption {
                inner: Some((inner, ttl)),
            },
            None => TimedOption::none(),
        }
    }
}

impl<T> From<TimedOption<T>> for Option<T> {
    fn from(value: TimedOption<T>) -> Self {
        value.into_option()
    }
}

/// See [module level documentation][crate]
#[derive(Debug, Copy, Clone)]
pub enum TimedValue<T> {
    Valid(T),
    Expired(T),
    None,
}

impl<T> TimedValue<T> {
    /// Returns `true` if the TimedValue is `Valid`.
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

impl<T> From<TimedOption<T>> for TimedValue<T> {
    fn from(value: TimedOption<T>) -> Self {
        value.into_timed_value()
    }
}
