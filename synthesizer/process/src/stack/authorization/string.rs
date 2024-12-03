// Copyright 2024 Aleo Network Foundation
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use super::*;

impl<N: Network> FromStr for Authorization<N> {
    type Err = Error;

    /// Initializes the authorization from a JSON-string.
    fn from_str(authorization: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(authorization)?)
    }
}

impl<N: Network> Debug for Authorization<N> {
    /// Prints the authorization as a JSON-string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network> Display for Authorization<N> {
    /// Displays the authorization as a JSON-string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).map_err::<fmt::Error, _>(ser::Error::custom)?)
    }
}

// sort inner transitions with the order of the inner request's program id
// it serves for the authorization building in wasm sdk
impl<N: Network> Authorization<N> {
    pub fn sort_transitions(&mut self) {
        let mut filter = HashMap::with_capacity(self.transitions.read().len());
        for transition in self.transitions().into_values() {
            filter.insert(transition.program_id().to_string(), transition);
        }

        let mut ordered_transitions = IndexMap::new();
        for request in self.requests.read().iter() {
            let transition = filter.get(&request.program_id().to_string()).expect("program id mismatched");
            ordered_transitions.insert(transition.id().clone(), transition.clone());
        }

        self.transitions = Arc::new(RwLock::new(ordered_transitions));
    }
}
