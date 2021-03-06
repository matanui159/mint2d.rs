// Copyright 2018 Joshua Minter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

include!(concat!(env!("OUT_DIR"), "/gl.rs"));

pub trait CheckError {
	fn check_error(&self);
}

impl CheckError for Gl {
	fn check_error(&self) {
		unsafe {
			let error = self.GetError();
			if error != NO_ERROR {
				panic!("glGetError = 0x{:X}", error)
			}
		}
	}
}