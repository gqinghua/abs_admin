pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
        let digest = md5::compute(raw_password);
        format!("{:x}", digest)
    }
    pub fn verify(password: &str, raw_password: &str) -> bool {
        let hashed = PasswordEncoder::encode(raw_password);
        password.eq(&hashed)
    }
}

///测试模块
#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    ///测试密码 编码和解码
    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456");
        println!("{}", s);
        assert_eq!(PasswordEncoder::encode("123456"), PasswordEncoder::encode("123456"))
    }
}