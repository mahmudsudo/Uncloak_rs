pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// vignere encryption
pub fn encrypt(key: &str, plaintext: &str) -> String {
    // encrypt
    let key_it = key.bytes().cycle();
    String::from_utf8(
        std::iter::zip(plaintext.bytes(), key_it)
            .map(|(p, k)| {
                let p = p - b'a';
                let k = k - b'a';
                let c = (p + k) % 26;
                c + b'a'
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
pub fn decrypt(key: &str, ciphertext: &str) -> String {
    let key_it = key.bytes().cycle();
    String::from_utf8(
        std::iter::zip(ciphertext.bytes(), key_it)
            .map(|(p, k)| {
                let p = p - b'a';
                let k = k - b'a';
                let c = (26 + p - k) % 26;
                c + b'a'
            })
            .collect(),
    )
    .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vig() {
      let msg = "aoeuidhtnsqjkxbmwvzpyfgcrl";
      let key = "averygoodkey";
      
      let ciphertext = encrypt(key,msg);
      
      let plaintext = decrypt(key, &ciphertext);
      dbg!(ciphertext);
      assert_eq!(msg, plaintext);
    
  }
}
