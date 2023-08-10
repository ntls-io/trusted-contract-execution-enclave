#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the mean of an array of integers passed in as a JSON array. Return value will be rounded down.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> i32 {
    // Print the received data and msg_len to verify their values
    println!("Received data: {:?}", unsafe {
        std::slice::from_raw_parts(msg, msg_len as usize)
    });
    println!("Received msg_len: {}", msg_len);

    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };
    // TODO: Fix error handling
    let val: Vec<i32> = match serde_json::from_slice(&x) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error deserializing JSON: {}", err);
            // Return a default value or handle the error appropriately.
            // For simplicity, let's return 0.
            return 0;
        }
    };

    let sum: i32 = val.iter().sum();
    sum / val.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Error;
    use std::env;
    use std::fs::File;
    use std::io::Read;

    /// Read data from the JSON file and parse it into a vector of floats.
    fn read_data_from_json(test_name: &str) -> Result<Vec<i32>, Error> {
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

        let data_vec: Vec<i32> = test_data
            .iter()
            .map(|v| v.as_i64().unwrap() as i32)
            .collect();
        println!("Parsed data: {:?}", data_vec);

        Ok(data_vec)
    }

    #[test]
    fn mean_int_works() {
        let data = read_data_from_json("mean_int_works").unwrap();
        println!("{:?}", data);

        // Ensure that the data vector has the correct size
        assert_eq!(data.len(), 7);

        // Create a new byte array that holds the serialized JSON data
        let serialized_data: Vec<u8> = serde_json::to_vec(&data).unwrap();

        // Call the exec function with the correct data size and data array
        let res = unsafe { exec(serialized_data.as_ptr(), serialized_data.len() as u32) };
        println!("Calculated mean: {}", res);

        // The mean of [8, 6, 8, 3, 7, 1, 9] is 6
        assert_eq!(res, 6);
    }

    #[test]
    fn mean_int_works_round() {
        let data = read_data_from_json("mean_int_works_decimal").unwrap();
        println!("{:?}", data);

        // Ensure that the data vector has the correct size
        assert_eq!(data.len(), 8);

        // Create a new byte array that holds the serialized JSON data
        let serialized_data: Vec<u8> = serde_json::to_vec(&data).unwrap();

        // Call the exec function with the correct data size and data array
        let res = unsafe { exec(serialized_data.as_ptr(), serialized_data.len() as u32) };
        println!("Calculated mean: {}", res);
        assert_eq!(res, 4);
    }
}
