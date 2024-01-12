
mod transport_layer;
pub mod application_layer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::transport_layer::socket::Socket;

    #[test]
    fn bind_socket() {
        let result = Socket::bind("127.0.0.1:12345");
        assert!(result.is_ok());
    }
}
