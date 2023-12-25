#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SpaceMission {
    id: u64,
    mission_name: String,
    destination: String,
    launch_date: u64,
    estimated_arrival_date: u64,
    description: String,
    // Add more fields as needed
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct SpaceMissionPayload {
    mission_name: String,
    destination: String,
    launch_date: u64,
    estimated_arrival_date: u64,
    description: String,
    // Add more fields as needed
}

impl Storable for SpaceMission {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SpaceMission {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Existing thread-local variables
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static SPACE_MISSION_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for space missions")
    );

    static SPACE_MISSION_STORAGE: RefCell<StableBTreeMap<u64, SpaceMission, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for SpaceMission
fn do_insert_space_mission(item: &SpaceMission) {
    SPACE_MISSION_STORAGE.with(|service| {
        service.borrow_mut().insert(item.id, item.clone());
    });
}

#[ic_cdk::query]
fn get_space_mission(id: u64) -> Result<SpaceMission, Error> {
    match _get_space_mission(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("space mission with id={} not found", id),
        }),
    }
}

fn _get_space_mission(id: &u64) -> Option<SpaceMission> {
    SPACE_MISSION_STORAGE.with(|s| s.borrow().get(id))
}

#[ic_cdk::update]
fn add_space_mission(item: SpaceMissionPayload) -> Option<SpaceMission> {
    let id = SPACE_MISSION_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for space missions");
    let space_mission = SpaceMission {
        id,
        mission_name: item.mission_name,
        destination: item.destination,
        launch_date: item.launch_date + 1 * 24 * 60 * 60 * 1_000_000_000 + ic_cdk::api::time(), // convert days to nanoseconds
        estimated_arrival_date: item.estimated_arrival_date
            + 1 * 24 * 60 * 60 * 1_000_000_000
            + ic_cdk::api::time(), // convert days to nanoseconds
        description: item.description,
        // Add more fields as needed
    };
    do_insert_space_mission(&space_mission);
    Some(space_mission)
}

#[ic_cdk::update]
fn update_space_mission(id: u64, item: SpaceMissionPayload) -> Result<SpaceMission, Error> {
    match SPACE_MISSION_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut space_mission) => {
            space_mission.mission_name = item.mission_name;
            space_mission.destination = item.destination;
            space_mission.launch_date = item.launch_date;
            space_mission.estimated_arrival_date = item.estimated_arrival_date;
            space_mission.description = item.description;
            // Update more fields as needed
            do_insert_space_mission(&space_mission);
            Ok(space_mission)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update space mission with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn delete_space_mission(id: u64) -> Result<SpaceMission, Error> {
    match SPACE_MISSION_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(space_mission) => Ok(space_mission),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete space mission with id={}. item not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_space_missions_before_date(estimated_arrival_date: u64) -> Vec<SpaceMission> {
    SPACE_MISSION_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, mission)| mission.estimated_arrival_date <= estimated_arrival_date)
            .map(|(_, mission)| mission.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_estimated_arrival_date(
    id: u64,
    new_estimated_arrival_date: u64,
) -> Result<SpaceMission, Error> {
    match SPACE_MISSION_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut space_mission) => {
            space_mission.estimated_arrival_date = new_estimated_arrival_date;
            do_insert_space_mission(&space_mission);
            Ok(space_mission)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update estimated arrival date for space mission with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_all_space_missions() -> Vec<SpaceMission> {
    SPACE_MISSION_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_total_space_missions() -> u64 {
    SPACE_MISSION_STORAGE.with(|service| service.borrow().len())
}

#[ic_cdk::query]
fn get_space_missions_count_before_date(estimated_arrival_date: u64) -> usize {
    SPACE_MISSION_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, mission)| mission.estimated_arrival_date <= estimated_arrival_date)
            .count()
    })
}

#[ic_cdk::query]
fn find_closest_space_mission(target_date: u64) -> Option<SpaceMission> {
    SPACE_MISSION_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, mission)| mission.estimated_arrival_date >= target_date)
            .min_by_key(|(_, mission)| mission.estimated_arrival_date)
            .map(|(_, mission)| mission.clone())
    })
}

#[ic_cdk::query]
fn calculate_remaining_days_for_arrival(id: u64) -> Result<u64, Error> {
    match SPACE_MISSION_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(space_mission) => {
            let current_date = ic_cdk::api::time();
            if space_mission.estimated_arrival_date > current_date {
                Ok(space_mission.estimated_arrival_date - current_date)
            } else {
                Err(Error::NotFound {
                    msg: "Space Mission has already arrived".to_string(),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("Couldn't find Space Mission with id={}. Item not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_space_missions_by_destination(destination: String) -> Vec<SpaceMission> {
    SPACE_MISSION_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, mission)| mission.destination == destination)
            .map(|(_, mission)| mission.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_launch_date(id: u64, new_launch_date: u64) -> Result<SpaceMission, Error> {
    match SPACE_MISSION_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut space_mission) => {
            space_mission.launch_date = new_launch_date;
            do_insert_space_mission(&space_mission);
            Ok(space_mission)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "Couldn't update launch date for Space Mission with id={}. Item not found",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
