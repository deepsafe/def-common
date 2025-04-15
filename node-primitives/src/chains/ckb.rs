// This file is part of DeepSafe.

// Copyright (C) DeepSafe (HK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sp_core::bounded::alloc::string::String;
use crate::chains::btc::verify_btc_ecdsa;

pub fn verify_ckb_ecdsa(pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<(), String> {
    verify_btc_ecdsa(pubkey, msg, sig)
}
