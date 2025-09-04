// Import necessary traits and modules
use serde::{Deserialize, Serialize};

// Define a struct to represent the data we want to serialize.
// The `derive(Serialize, Deserialize)` attributes automatically
// generate the code needed for serde to work its magic.
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    age: u8,
    is_active: bool,
    metadata: Option<String>,
}

fn main() {
    // 1. Create an instance of the struct you want to serialize.
    let my_user = User {
        username: "ferris".to_string(),
        age: 8,
        is_active: true,
        metadata: Some("A friendly crab".to_string()),
    };

    println!("Original user struct: {:?}", my_user);
    println!("--------------------------------------");

    // 2. Serialize the struct into a CBOR byte vector.
    // The `to_vec` function handles the entire serialization process.
    // This will return a `Result<Vec<u8>, ...>`, so we unwrap it for simplicity.
    let cbor_bytes = serde_cbor::to_vec(&my_user).unwrap();

    println!("Serialized CBOR bytes: {:?}", cbor_bytes);
    println!("Size of CBOR object: {} bytes", cbor_bytes.len());
    println!("--------------------------------------");

    // 3. Deserialize the CBOR bytes back into a User struct.
    // The `from_slice` function takes a byte slice and tries to deserialize it.
    let deserialized_user: User = serde_cbor::from_slice(&cbor_bytes).unwrap();

    println!("Deserialized user struct: {:?}", deserialized_user);
    println!("--------------------------------------");

    // You can also demonstrate the `None` case for optional fields.
    let another_user = User {
        username: "rustacean".to_string(),
        age: 5,
        is_active: false,
        metadata: None, // No metadata for this user
    };

    let another_cbor_bytes = serde_cbor::to_vec(&another_user).unwrap();
    let deserialized_another_user: User = serde_cbor::from_slice(&another_cbor_bytes).unwrap();
    
    println!("Deserialized user with None metadata: {:?}", deserialized_another_user);
}
