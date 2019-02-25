// Copyright 2014 Carl Lerche, Steve Klabnik, Alex Crichton, Yehuda Katz,
//                Ben Longbons
// Copyright 2015 Carl Lerche, Alex Crichton, Robin Gloster
// Copyright 2016 Urban Hafner
// Copyright 2018 Val Markovic
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Borrow;
use std::fmt;
use std::marker::PhantomData;

use core::*;

pub struct EqualTo<T, D> {
  expected: T,
  marker: PhantomData<D>,
}

impl<T, D> fmt::Display for EqualTo<T, D>
where
  T: fmt::Debug,
  D: fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.expected.fmt(f)
  }
}

impl<T, U, B> Matcher<B> for EqualTo<U, T>
where
  U: PartialEq<T> + fmt::Debug,
  T: fmt::Debug,
  B: Borrow<T>,
{
  fn matches(&self, actual: B) -> MatchResult {
    if self.expected.eq(actual.borrow()) {
      success()
    } else {
      Err(format!("was {:?}", actual.borrow()))
    }
  }
}

pub fn equal_to<T, D>(expected: T) -> EqualTo<T, D> {
  EqualTo {
    expected,
    marker: PhantomData,
  }
}
