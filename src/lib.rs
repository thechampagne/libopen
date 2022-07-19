/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use open::that;
use open::with;

/// Open path with the default application without blocking.
/// 
/// Example:
/// * *
/// int main()
/// {
///     if (open_that("http://rust-lang.org") != 0)
///     {
///         printf("An error occurred when opening http://rust-lang.org\n");
///         return -1;
///     }
///     printf("Opened successfully.");
///     return 0;
/// }
/// * *
/// 
/// @param path
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn open_that(path: *const c_char) -> c_int {
  if path.is_null() {
    return -1;
  }
  let str = match CStr::from_ptr(path).to_str() {
    Ok(s) => s,
    Err(_) => return -1
  };
  match that(str) {
    Ok(_) => 0,
    Err(_) => -1
  }
}

/// Open path with the given application.
/// 
/// Example:
/// * *
/// int main()
/// {
///     if (open_with("http://rust-lang.org", "firefox") != 0)
///     {
///         printf("An error occurred when opening http://rust-lang.org\n");
///         return -1;
///     }
///     printf("Opened successfully.");
///     return 0;
/// }
/// * *
/// 
/// @param path
/// @param app
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn open_with(path: *const c_char, app: *const c_char) -> c_int {
  if path.is_null() || app.is_null() {
    return -1;
  }
  let pstr = match CStr::from_ptr(path).to_str() {
    Ok(s) => s,
    Err(_) => return -1
  };
  let astr = match CStr::from_ptr(app).to_str() {
    Ok(s) => s,
    Err(_) => return -1
  };
  match with(pstr,astr) {
    Ok(_) => 0,
    Err(_) => -1
  }
}