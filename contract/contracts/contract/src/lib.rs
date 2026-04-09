#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, String, Address, Symbol, symbol_short
};

#[contract]
pub struct RideSharingContract;

#[contracttype]
#[derive(Clone)]
pub struct Ride {
    pub rider: Address,
    pub driver: Option<Address>,
    pub pickup: String,
    pub destination: String,
    pub status: Symbol,
}

#[contractimpl]
impl RideSharingContract {

    pub fn create_ride(
        env: Env,
        rider: Address,
        pickup: String,
        destination: String,
    ) -> u64 {
        let ride_id: u64 = env.ledger().sequence().into();

        let ride = Ride {
            rider,
            driver: None,
            pickup,
            destination,
            status: symbol_short!("REQ"),
        };

        env.storage().instance().set(&ride_id, &ride);

        ride_id
    }

    pub fn accept_ride(
        env: Env,
        ride_id: u64,
        driver: Address,
    ) {
        let mut ride: Ride = env.storage().instance().get(&ride_id).unwrap();

        ride.driver = Some(driver);
        ride.status = symbol_short!("ACC");

        env.storage().instance().set(&ride_id, &ride);
    }

    pub fn complete_ride(env: Env, ride_id: u64) {
        let mut ride: Ride = env.storage().instance().get(&ride_id).unwrap();

        ride.status = symbol_short!("DONE");

        env.storage().instance().set(&ride_id, &ride);
    }

    pub fn get_ride(env: Env, ride_id: u64) -> Ride {
        env.storage().instance().get(&ride_id).unwrap()
    }
}