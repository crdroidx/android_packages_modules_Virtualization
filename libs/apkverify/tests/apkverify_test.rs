/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use apkverify::{testing::assert_contains, verify};
use std::matches;

const KEY_NAMES_DSA: &[&str] = &["1024", "2048", "3072"];
const KEY_NAMES_ECDSA: &[&str] = &["p256", "p384", "p521"];
const KEY_NAMES_RSA: &[&str] = &["1024", "2048", "3072", "4096", "8192", "16384"];

#[test]
fn test_verify_truncated_cd() {
    use zip::result::ZipError;
    let res = verify("tests/data/v2-only-truncated-cd.apk");
    // TODO(b/190343842): consider making a helper for err assertion
    assert!(matches!(
        res.unwrap_err().root_cause().downcast_ref::<ZipError>().unwrap(),
        ZipError::InvalidArchive(_),
    ));
}

#[test]
fn test_verify_v3() {
    assert!(verify("tests/data/test.apex").is_ok());
}

#[test]
fn test_verify_v3_dsa_sha256() {
    for key_name in KEY_NAMES_DSA.iter() {
        let res = verify(format!("tests/data/v3-only-with-dsa-sha256-{}.apk", key_name));
        assert!(res.is_err());
        assert_contains(&res.unwrap_err().to_string(), "not implemented");
    }
}

#[test]
fn test_verify_v3_ecdsa_sha256() {
    for key_name in KEY_NAMES_ECDSA.iter() {
        assert!(verify(format!("tests/data/v3-only-with-ecdsa-sha256-{}.apk", key_name)).is_ok());
    }
}

#[test]
fn test_verify_v3_ecdsa_sha512() {
    for key_name in KEY_NAMES_ECDSA.iter() {
        let res = verify(format!("tests/data/v3-only-with-ecdsa-sha512-{}.apk", key_name));
        assert!(res.is_err());
        assert_contains(&res.unwrap_err().to_string(), "not implemented");
    }
}

#[test]
fn test_verify_v3_rsa_sha256() {
    for key_name in KEY_NAMES_RSA.iter() {
        assert!(
            verify(format!("tests/data/v3-only-with-rsa-pkcs1-sha256-{}.apk", key_name)).is_ok()
        );
    }
}

#[test]
fn test_verify_v3_rsa_sha512() {
    for key_name in KEY_NAMES_RSA.iter() {
        assert!(
            verify(format!("tests/data/v3-only-with-rsa-pkcs1-sha512-{}.apk", key_name)).is_ok()
        );
    }
}

#[test]
fn test_verify_v3_sig_does_not_verify() {
    let path_list = [
        "tests/data/v3-only-with-dsa-sha256-2048-sig-does-not-verify.apk",
        "tests/data/v3-only-with-ecdsa-sha512-p521-sig-does-not-verify.apk",
        "tests/data/v3-only-with-rsa-pkcs1-sha256-3072-sig-does-not-verify.apk",
    ];
    for path in path_list.iter() {
        let res = verify(path);
        assert!(res.is_err());
        let error_msg = &res.unwrap_err().to_string();
        assert!(
            error_msg.contains("Signature is invalid") || error_msg.contains("not implemented")
        );
    }
}

#[test]
fn test_verify_v3_digest_mismatch() {
    let path_list = [
        "tests/data/v3-only-with-dsa-sha256-3072-digest-mismatch.apk",
        "tests/data/v3-only-with-rsa-pkcs1-sha512-8192-digest-mismatch.apk",
    ];
    for path in path_list.iter() {
        let res = verify(path);
        assert!(res.is_err());
        let error_msg = &res.unwrap_err().to_string();
        assert!(error_msg.contains("Digest mismatch") || error_msg.contains("not implemented"));
    }
}

#[test]
fn test_verify_v3_wrong_apk_sig_block_magic() {
    let res = verify("tests/data/v3-only-with-ecdsa-sha512-p384-wrong-apk-sig-block-magic.apk");
    assert!(res.is_err());
    assert_contains(&res.unwrap_err().to_string(), "No APK Signing Block");
}

#[test]
fn test_verify_v3_apk_sig_block_size_mismatch() {
    let res =
        verify("tests/data/v3-only-with-rsa-pkcs1-sha512-4096-apk-sig-block-size-mismatch.apk");
    assert!(res.is_err());
    assert_contains(
        &res.unwrap_err().to_string(),
        "APK Signing Block sizes in header and footer do not match",
    );
}

#[test]
fn test_verify_v3_cert_and_public_key_mismatch() {
    let res = verify("tests/data/v3-only-cert-and-public-key-mismatch.apk");
    assert!(res.is_err());
    assert_contains(&res.unwrap_err().to_string(), "Public key mismatch");
}

#[test]
fn test_verify_v3_empty() {
    let res = verify("tests/data/v3-only-empty.apk");
    assert!(res.is_err());
    assert_contains(&res.unwrap_err().to_string(), "APK too small for APK Signing Block");
}

#[test]
fn test_verify_v3_no_certs_in_sig() {
    let res = verify("tests/data/v3-only-no-certs-in-sig.apk");
    assert!(res.is_err());
    assert_contains(&res.unwrap_err().to_string(), "No certificates listed");
}

#[test]
fn test_verify_v3_no_supported_sig_algs() {
    let res = verify("tests/data/v3-only-no-supported-sig-algs.apk");
    assert!(res.is_err());
    assert_contains(&res.unwrap_err().to_string(), "No supported signatures found");
}

#[test]
fn test_verify_v3_signatures_and_digests_block_mismatch() {
    let res = verify("tests/data/v3-only-signatures-and-digests-block-mismatch.apk");
    assert!(res.is_err());
    assert_contains(
        &res.unwrap_err().to_string(),
        "Signature algorithms don't match between digests and signatures records",
    );
}

#[test]
fn test_verify_v3_unknown_additional_attr() {
    assert!(verify("tests/data/v3-only-unknown-additional-attr.apk").is_ok());
}

#[test]
fn test_verify_v3_unknown_pair_in_apk_sig_block() {
    assert!(verify("tests/data/v3-only-unknown-pair-in-apk-sig-block.apk").is_ok());
}

#[test]
fn test_verify_v3_ignorable_unsupported_sig_algs() {
    assert!(verify("tests/data/v3-only-with-ignorable-unsupported-sig-algs.apk").is_ok());
}

#[test]
fn test_verify_v3_stamp() {
    assert!(verify("tests/data/v3-only-with-stamp.apk").is_ok());
}
