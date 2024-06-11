use chrono::{NaiveDateTime, TimeZone, Utc};

pub fn timestamp_to_epoch(timestamp: String) -> Result<u64, String> {
    // Split the input timestamp into date and time parts
    let parts: Vec<&str> = timestamp.split(' ').collect();
    if parts.len() != 2 {
        return Err("Invalid timestamp format".to_string());
    }

    let date_str = parts[0];
    let time_str = parts[1];

    // Parse the input date and time separately using NaiveDateTime from the chrono crate
    match NaiveDateTime::parse_from_str(&format!("{} {}", date_str, time_str), "%Y-%m-%d %H:%M:%S") {
        Ok(naive_date_time) => {

            let date_time_utc = Utc.from_utc_datetime(&naive_date_time);
            let epoch_millis = date_time_utc.timestamp_millis();
            Ok(epoch_millis as u64)
        }
        Err(e) => Err(format!("Failed to parse timestamp: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_test_valid_timestamp() {
        let timestamp: String = "2024-06-23 4:22:22".to_string();
        let expected_epoch: u64 = 1719116542000;

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => assert_eq!(epoch, expected_epoch),
            Err(e) => panic!("Expected Ok({}) but got ERR({})", expected_epoch, e),
        }
    }

    #[test]
    fn conversion_test_no_space() {
        let timestamp: String = "2024-11-2103:33:32".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
            Err(e) => assert_eq!(e, "Invalid timestamp format".to_string()),
        }
    }

    #[test]
    fn conversion_test_empty_timestamp() {
        let timestamp = "".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
            Err(e) => assert_eq!(e, "Invalid timestamp format".to_string()),
        }
    }

    #[test]
    fn conversion_test_invalid_format() {
        let timestamp = "2023/05/30 12:34:56".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
            Err(e) => assert!(e.contains("Failed to parse timestamp")),
        }
    }

    #[test]
    fn conversion_test_invalid_date() {
        let timestamp: String = "2024-55-41 12:33:33".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Invalid timestamp format, got {}", epoch),
            Err(e) => assert!(e.contains("Failed to parse timestamp")),
        }
    }

    #[test]
    fn conversion_test_invalid_time() {
        let timestamp: String = "2024-10-30 24:41:77".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Invalid timestamp format, got {}", epoch),
            Err(e) => assert!(e.contains("Failed to parse timestamp")),
        }
    }

    #[test]
    fn conversion_test_valid_timestamp_2() {
        let timestamp: String = "2027-02-05 02:04:38".to_string();
        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => {
                // Verify the epoch value is correct
                let expected_epoch: u64 = 1801793078000;
                assert_eq!(epoch, expected_epoch);
            },
            Err(e) => panic!("Expected valid date, but got error: {}", e),
        }
    }
}