pub mod application_layer;
pub mod transport_layer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::transport_layer::socket::SocketAbstraction;

    #[test]
    fn bind_socket() {
        let result = SocketAbstraction::bind("127.0.0.1:12345");
        assert!(result.is_ok());
    }
}
