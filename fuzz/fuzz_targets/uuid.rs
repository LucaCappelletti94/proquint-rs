//! Fuzzing submodule for testing correctness of the implementation of the [`Quintable`](proquint::Quintable)
//! trait for the [`Uuid`](uuid::Uuid) type.

use honggfuzz::fuzz;
use proquint::Quintable;
use uuid::Uuid;

fn main() {
    loop {
        fuzz!(|uuid: uuid::Uuid| {
            // Convert the UUID to a quint string
            let quint = uuid.to_quint();

            // Convert the quint string back to a UUID
            let result = Uuid::from_quint(&quint).expect("Failed to convert back");

            // Check if the conversion was successful
            assert_eq!(uuid, result, "UUID conversion failed for: {}", uuid);
        });
    }
}
