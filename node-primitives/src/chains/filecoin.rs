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

use sp_core::bounded::alloc::{string::{ToString, String}, format};
use def_chain_bridge::utils::disintegrate_fil_msg;
use secp256k1::Message;
use crate::bls_verify;

pub fn verify_filecoin(pubkey: &[u8], raw: &[u8], sig: &[u8], engine: &str) -> Result<(), String> {
    let msg_vec = match disintegrate_fil_msg(&hex::encode(raw), engine) {
        Ok(param) => param.1,
        Err(e) => {
            return Err(e);
        }
    };
    match engine {
        "ECDSA" => {
            let mut msg = [0; 32];
            msg.copy_from_slice(&msg_vec);
            let message = Message::parse(&msg);
            let signature = secp256k1::Signature::parse_slice(&sig[..64]).map_err(|e| e.to_string())?;
            let pubkey = secp256k1::PublicKey::parse_slice(pubkey, None).map_err(|e| e.to_string())?;
            if !secp256k1::verify(&message, &signature, &pubkey) {
                return Err("filecoin ecdsa signature verify failed".to_string());
            }
        }
        "BLS" => {
            let mut msg = [0; 38];
            msg.copy_from_slice(&msg_vec);
            bls_verify(pubkey, &msg, sig).map_err(|e| format!("filecoin bls signature verify failed for: {e:?}"))?;
        }
        _ => return Err(format!("unsupport engine: {engine:?} to verify filecoin signature"))
    }
    Ok(())
}
