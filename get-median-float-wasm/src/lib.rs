#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the median of an array of floats passed in as a JSON array.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> f32 {
    // Print the received data and msg_len to verify their values
    println!("Received data: {:?}", unsafe {
        std::slice::from_raw_parts(msg, msg_len as usize)
    });
    println!("Received msg_len: {}", msg_len);
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };

    //TODO - Fix error handling
    let mut val: Vec<f32> = match serde_json::from_slice(&x) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error deserializing JSON: {}", err);
            // Return a default value or handle the error appropriately.
            // For simplicity, let's return 0.
            return 0.0;
        }
    };
    val.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let val_len = val.len();
    if val_len % 2 == 0 {
        let mid = val_len / 2;
        (val[mid - 1] + val[mid]) / 2.0
    } else {
        let mid = (val_len + 1) / 2;
        val[mid - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Error;
    use std::env;
    use std::fs::File;
    use std::io::Read;

    /// Read data from the JSON file and parse it into a vector of floats.
    fn read_data_from_json(test_name: &str) -> Result<Vec<f32>, Error> {
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push("test_data.json");
        println!("{:?}", current_dir);
        let mut file = File::open(&current_dir).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        println!("Raw JSON data for {}: {}", test_name, data);

        let json_data: serde_json::Value = serde_json::from_str(&data)?;

        // Check if the JSON data is correctly parsed
        println!("Parsed JSON data: {:?}", json_data);

        let test_data = json_data[test_name].as_array().unwrap();
        println!("Test data: {:?}", test_data);

        let data_vec: Vec<f32> = test_data
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect();
        println!("Parsed data: {:?}", data_vec);

        Ok(data_vec)
    }

    #[test]
    fn median_float_works_odd() {
        let data: Vec<f32> = read_data_from_json("median_float_works_odd").unwrap();
        println!("{:?}", data);
        // Ensure that the data vector has the correct size
        assert_eq!(data.len(), 7);
        // Create a new byte array that holds the serialized JSON data
        let serialized_data = serde_json::to_vec(&data).unwrap();

        let res = unsafe { exec(serialized_data.as_ptr(), serialized_data.len() as u32) };
        println!("Calculated median: {}", res);
        assert_eq!(res, 6.0);
    }

    #[test]
    fn median_float_works_even() {
        let data: Vec<f32> = read_data_from_json("median_float_works_even").unwrap();
        println!("{:?}", data);
        // Ensure that the data vector has the correct size
        assert_eq!(data.len(), 8);
        // Create a new byte array that holds the serialized JSON data
        let serialized_data = serde_json::to_vec(&data).unwrap();

        let res = unsafe { exec(serialized_data.as_ptr(), serialized_data.len() as u32) };
        println!("Calculated median: {}", res);
        assert_eq!(res, 4.5);
    }
}
