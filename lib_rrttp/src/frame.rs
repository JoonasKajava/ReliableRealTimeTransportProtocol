
pub struct Frame([u8]);

impl Frame {

    pub fn calculate_checksum(&self) -> usize {
        let mut sum: usize = 0;
        for byte in &self.0 {
            sum += *byte as usize;
            if (sum & 0xFF00) > 0 {
                sum &= 0xFF;
                sum += 1;
            }
        }
        sum & 0xFF
    }

    pub fn verify_checksum(&self) -> bool {
        let checksum = &self.0[0]+&self.0[0];
    }
}