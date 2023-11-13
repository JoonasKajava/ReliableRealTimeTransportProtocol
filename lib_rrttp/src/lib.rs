pub mod socket;
pub mod constants;
pub mod window;
pub mod frame;
pub mod control_bits;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::socket::Socket;

    #[test]
    fn bind_socket() {
        let result = Socket::bind("127.0.0.1:12345");
        assert!(result.is_ok());
    }
}
