const CUSTOM_CRC_ALGO: crc::Algorithm<u32> = crc::Algorithm {
    width: 32,
    poly: 0x04C11DB7,
    init: 0xffffffff,
    refin: true,
    refout: true,
    xorout: 0xffffffff,
    check: 0xaee7,
    residue: 0x0000,
};

pub fn calc_crc32(crc_input: &[u8; 9]) -> u32 {
    let crc = crc::Crc::<u32>::new(&CUSTOM_CRC_ALGO);
    let mut digest = crc.digest();
    digest.update(crc_input);
    digest.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        let crc_input = b"123456789";
        let crc_result = calc_crc32(crc_input);
        assert_eq!(0xCBF43926, crc_result);
    }
}
