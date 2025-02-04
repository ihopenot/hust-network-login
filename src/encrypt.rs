use std::str::FromStr;

use num_bigint::BigUint;

pub fn encrypt_pass(password: String) -> String {
    let e: u32 = 65537;
    let n  = BigUint::parse_bytes(b"94dd2a8675fb779e6b9f7103698634cd400f27a154afa67af6166a43fc26417222a79506d34cacc7641946abda1785b7acf9910ad6a0978c91ec84d40b71d2891379af19ffb333e7517e390bd26ac312fe940c340466b4a5d4af1d65c3b5944078f96a1a51a5a53e4bc302818b7c9f63c4a1b07bd7d874cef1c3d4b2f5eb7871",16).unwrap();
    let c = BigUint::from_bytes_be(&password.as_bytes());
    let result = c.modpow(&BigUint::from(e), &n);
    result.to_str_radix(16)
}

#[test]

fn encrypt_test() {
    assert_eq!(encrypt_pass(String::from_str("123>5ae915bf808f82732e98e01f704f00cd").unwrap()),"91a0e02175f6a0b22ad23dac0d7f599806bc091f9fee1bfdada0d24d011dcdaed418296b7c0ec560f988d92a7bb25dbf7ff51752d9bc6482a8180e56f7b772079ab59844abaae91e6d1c4660dc872717f9218f89acc9b70bb32891f28bf9d8f173d81b0e36c828deac919783e4e909ad1c22f953947b4a7ed7c90ac18fd95aa2");
}
