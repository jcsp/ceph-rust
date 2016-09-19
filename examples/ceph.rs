// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_imports)]

extern crate ceph_rust;
extern crate lsio;

use ceph_rust as ceph;

#[cfg(not(target_os = "linux"))]
fn main(){}

#[cfg(target_os = "linux")]
fn main() {
  let mut major: i32 = 0;
  let mut minor: i32 = 0;
  let mut extra: i32 = 0;

  unsafe {
    ceph::rados_version(&mut major, &mut minor, &mut extra);
  }

  println!("v{}.{}.{}", major, minor, extra);

}
