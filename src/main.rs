/*
 * The MIT License (MIT)
 * Copyright (c) 2018 Joern Huxhorn
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

/*
 * Copyright 2018 Joern Huxhorn
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(missing_docs)]
#![allow(dead_code)]
#![forbid(unsafe_code)]

//! # Command line tool for generating and validating ULIDs

use rusty_ulid::Ulid;
use std::str::FromStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");
static HELP: &str = "rusty_ulid

Usage:
    rusty_ulid [options]
        Generate a ULID.

    rusty_ulid [options] <args>...
        Check ULIDs given as args.

Options:
    -h, --help          Display this message and exit
    -V, --version       Print version info and exit
    -v, --verbose       Use verbose output
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exit_code = main_with_args_and_return_value(args);
    std::process::exit(exit_code);
}

#[cfg(all(feature = "rand", any(feature = "chrono", feature = "time")))]
fn generate_ulid(verbose: bool) -> i32 {
    let ulid = Ulid::generate();
    print(&ulid, verbose);

    0
}

#[cfg(not(all(feature = "rand", any(feature = "chrono", feature = "time"))))]
fn generate_ulid(_verbose: bool) -> i32 {
    println!("Generation of ULID not supported.");

    1
}

fn print(ulid: &Ulid, verbose: bool) {
    if verbose {
        #[cfg(all(feature = "chrono", not(feature = "time")))]
        {
            use chrono::SecondsFormat;

            println!(
                "{}\n{}\n",
                ulid,
                ulid.datetime().to_rfc3339_opts(SecondsFormat::Millis, true)
            );
        }
        #[cfg(feature = "time")]
        {
            use time::format_description::well_known::Rfc3339;

            println!(
                "{}\n{}\n",
                ulid,
                ulid.offsetdatetime().format(&Rfc3339).unwrap()
            );
        }
    } else {
        println!("{}", ulid);
    }
}

fn main_with_args_and_return_value(args: Vec<String>) -> i32 {
    let mut verbose: bool = false;
    let mut help: bool = false;
    let mut version: bool = false;
    let mut ulid_candidates = Vec::<String>::new();

    for arg in args {
        let argument: &str = &arg;
        match argument {
            "-v" => verbose = true,
            "--verbose" => verbose = true,
            "-h" => help = true,
            "--help" => help = true,
            "-V" => version = true,
            "--version" => version = true,
            _ => ulid_candidates.push(argument.to_string()),
        }
    }

    if version {
        println!("rusty_ulid {}", VERSION);
        return 0;
    }

    if help {
        println!("{}", HELP);
        return 0;
    }

    if ulid_candidates.is_empty() {
        // not checking, producing
        return generate_ulid(verbose);
    }

    let mut broken = Vec::<String>::new();
    for candidate in ulid_candidates {
        let result = Ulid::from_str(&candidate);
        if let Ok(ulid) = result {
            if verbose {
                print(&ulid, verbose);
            }
        } else {
            broken.push(candidate);
        }
    }

    if !broken.is_empty() {
        eprintln!("Invalid ULID strings: {:?}", broken);
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_value_returns_error() {
        let args = vec!["foo".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 1);
    }

    #[test]
    fn valid_values_return_no_error() {
        let args = vec![
            "01CB265DSMTDS096TBTZRNTBPC".to_string(),
            "01CB265J6CRQA44WH98DP3YA07".to_string(),
        ];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_verbose_values_return_no_error() {
        let args = vec![
            "01CB265DSMTDS096TBTZRNTBPC".to_string(),
            "--verbose".to_string(),
            "01CB265J6CRQA44WH98DP3YA07".to_string(),
        ];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[cfg(not(miri))] // libc::gettimeofday
    #[cfg(all(feature = "rand", any(feature = "chrono", feature = "time")))]
    #[test]
    fn no_args_return_no_error() {
        let args = vec![];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[cfg(not(miri))] // libc::gettimeofday
    #[cfg(all(feature = "rand", any(feature = "chrono", feature = "time")))]
    #[test]
    fn verbose_short_returns_no_error() {
        let args = vec!["-v".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[cfg(not(miri))] // libc::gettimeofday
    #[cfg(all(feature = "rand", any(feature = "chrono", feature = "time")))]
    #[test]
    fn verbose_long_returns_no_error() {
        let args = vec!["--verbose".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[cfg(not(all(feature = "rand", any(feature = "chrono", feature = "time"))))]
    #[test]
    fn no_args_return_no_error() {
        let args = vec![];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 1);
    }

    #[cfg(not(all(feature = "rand", any(feature = "chrono", feature = "time"))))]
    #[test]
    fn verbose_short_returns_no_error() {
        let args = vec!["-v".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 1);
    }

    #[cfg(not(all(feature = "rand", any(feature = "chrono", feature = "time"))))]
    #[test]
    fn verbose_long_returns_no_error() {
        let args = vec!["--verbose".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 1);
    }

    #[test]
    fn version_short_returns_no_error() {
        let args = vec!["-V".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn version_long_returns_no_error() {
        let args = vec!["--version".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn help_short_returns_no_error() {
        let args = vec!["-h".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn help_long_returns_no_error() {
        let args = vec!["--help".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }
}
