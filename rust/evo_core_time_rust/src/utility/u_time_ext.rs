pub struct UTimeExt;

impl UTimeExt {
    pub fn to_local_time(time_ns:u64, time_fromat:Option<&str>) -> String{
        // Convert UTC nanoseconds to local time in format yyyy::mm::dd hh:mm:ss

        let format_str = match time_fromat {
            Some(format_str) => format_str,
            None => "%Y::%m::%d %H:%M:%S",
        };

        let timestamp = {

            // Convert nanoseconds to seconds for DateTime creation
            let seconds = time_ns / 1_000_000_000;
            let nanoseconds = (time_ns % 1_000_000_000) as u32;

            // Create DateTime from timestamp and format as yyyy::mm::dd hh:mm:ss
            if let Some(datetime) =
                chrono::DateTime::from_timestamp(seconds as i64, nanoseconds)
            {
                datetime
                    .with_timezone(&chrono::Local)
                    .format(format_str)
                    .to_string()
            } else {
                // Fallback to current time if timestamp is invalid
                chrono::Local::now()
                    .format(format_str)
                    .to_string()
            }
        };

        timestamp

    }
}
