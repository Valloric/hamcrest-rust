// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate hamcrest;

mod existing_path {

    pub use hamcrest::prelude::*;
    pub use std::env;
    pub use std::path::Path;
    pub use std::path::PathBuf;

    #[test]
    fn an_existing_file() {
        let path = path(env::var("TEST_EXISTS_FILE"), "./README.md");
        let matcher = is(existing_path());
        assert_that!(&path, matcher);
        let matcher2 = is(existing_file());
        assert_that!(&path, matcher2);
        let matcher3 = is_not(existing_dir());
        assert_that!(&path, matcher3);
    }

    #[test]
    fn an_existing_dir() {
        let path = path(env::var("TEST_EXISTS_DIR"), "./target");
        let matcher = is(existing_path());
        assert_that!(&path, matcher);
        let matcher2 = is(existing_dir());
        assert_that!(&path, matcher2);
        let matcher3 = is_not(existing_file());
        assert_that!(&path, matcher3);
    }

    #[test]
    fn a_nonexisting_path() {
        let path = path(env::var("TEST_EXISTS_NONE"), "./zomg.txt");
        let matcher = is_not(existing_path());
        assert_that!(&path, matcher);
        let matcher2 = is_not(existing_file());
        assert_that!(&path, matcher2);
        let matcher3 = is_not(existing_dir());
        assert_that!(&path, matcher3);
    }

    pub fn path(path: Result<String, env::VarError>, default: &str) -> PathBuf {
        Path::new(&path.unwrap_or(default.to_string())).to_owned()
    }

}
