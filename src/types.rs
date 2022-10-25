use serde::{Deserialize, Serialize};
pub use trinci_sdk::tai::AssetTransferArgs as TransferArgs;
use trinci_sdk::{WasmError, WasmResult};

// Environment variables
pub const CONFIG_KEY: &str = "config";
pub const INIT_KEY: &str = "init";

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Default))] // used to have Clone but trait derived upper
pub struct InitArgs<'a> {
    // Plane name.
    pub plane_name: &'a str,
    // Plane description.
    pub descrpition: &'a str,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone, Default))]
pub struct PlaneConfig<'a> {
    // Plane name.
    pub plane_name: &'a str,
    // Account that has invoked the `init` method.
    pub creator: &'a str,
    // Plane description.
    pub descrpition: &'a str,
}

impl<'a> PlaneConfig<'a> {
    pub fn new(creator: &'a str, init_args: InitArgs<'a>) -> Self {
        PlaneConfig {
            plane_name: init_args.plane_name,
            creator,
            descrpition: init_args.descrpition,
        }
    }
}

// Resource
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub enum ResourceType {
    Wood,
    Stone,
    Food,
    Magic, // Easter Egg, should happen only if drad brake.
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct Resource {
    // Type of resource.
    pub resource_type: ResourceType,
    // Minimum of resource that can be collected each time.
    pub minimum_collectable: u64,
    // Maximum of resource that can be collected each time.
    pub maximum_collectable: u64,
    // Last collection time (different foreach block).
    pub last_collection: u64,
}

impl Resource {
    pub fn new(
        resource_type: ResourceType,
        minimum_collectable: u64,
        maximum_collectable: u64,
    ) -> Self {
        Resource {
            resource_type,
            minimum_collectable,
            maximum_collectable,
            last_collection: 0,
        }
    }
}

/// Reserve in warehouse.
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone, Default))]
pub struct Reserve {
    pub wood: u64,
    pub stone: u64,
    pub food: u64,
    pub magic: u64,
}

impl Reserve {
    pub fn new() -> Self {
        Reserve {
            wood: 0,
            stone: 0,
            food: 0,
            magic: 0,
        }
    }
}

/// Warehouse, it holds the fortress collected resources.
// TODO: mabye implement a system of fortifiction,
//       the higher the harder to stole goods
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone, Default))]
pub struct Warehouse {
    pub reserve: Reserve,
}

impl Warehouse {
    pub fn new() -> Self {
        Warehouse {
            reserve: Reserve::new(),
        }
    }
}

/// Fortress
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct Fortress<'a> {
    // Fortress name.
    pub name: &'a str,
    // Fortress king.
    pub king: &'a str,
    // Fortress descriprion.
    pub description: &'a str,
    // Fortress divination number.
    pub divination_number: u8,
    // Fortress key resource.
    pub key_resource: Resource,
    // Frotress warehouse.
    pub warehouse: Warehouse,
}

impl Fortress<'_> {
    /// Constructor.
    pub fn new(init_args: InitFortressArgs) -> WasmResult<()> {
        let divination_number = (trinci_sdk::drand(8) + 1) as u8;

        let resource_type = {
            match trinci_sdk::drand(2) {
                0 => ResourceType::Wood,
                1 => ResourceType::Stone,
                2 => ResourceType::Food,
                _ => ResourceType::Magic,
            }
        };
        let maximum_collectable = trinci_sdk::drand(204) + 51; // In this way max is minimum 51.
        let minimum_collectable = trinci_sdk::drand(maximum_collectable - 1);

        let key_resource = Resource::new(resource_type, minimum_collectable, maximum_collectable);

        let fortress = Fortress {
            name: init_args.name,
            king: init_args.king,
            description: init_args.description,
            divination_number,
            key_resource,
            warehouse: Warehouse::new(),
        };

        trinci_sdk::store_account_data_mp!(CONFIG_KEY, &fortress)?;
        trinci_sdk::store_data(INIT_KEY, &[1]);

        Ok(())
    }

    /// Resource collector, if succeded the return value is the ammount collected.
    pub fn collect_resource(&self) -> WasmResult<u64> {
        // Check if already collected in the actual block
        if self
            .key_resource
            .last_collection
            .lt(&trinci_sdk::get_block_time())
        {
            // TODO: check if the  calculation logic works
            let max_to_collect =
                self.key_resource.maximum_collectable - self.key_resource.minimum_collectable;
            let resources_collected =
                trinci_sdk::drand(max_to_collect) + self.key_resource.minimum_collectable;

            // TODO: load data
            //       update collected resource
            //       save data

            Ok(resources_collected)
        } else {
            Err(WasmError::new(
                "Collection already completed in the actual block.",
            ))
        }
    }

    /// Add the specified resources to the forteress.
    pub fn add_resource(&self, resource: ResourceType, units: u64) -> WasmResult<()> {
        // TODO: load data
        //       update collected resource
        //       save data
        todo!()
    }

    /// Remove the specified resources to the forteress.
    pub fn remove_resource(&self, resource: ResourceType, units: u64) -> WasmResult<()> {
        // TODO: load data
        //       check if is doable (math stuff)
        //       update collected resource
        //       save data
        todo!()
    }
}

/// Fortress initialization arguments.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Default))] // used to have Clone but trait derived upper
pub struct InitFortressArgs<'a> {
    // Fortress name.
    pub name: &'a str,
    // Fortress king.
    pub king: &'a str,
    // Fortress descriprion.
    pub description: &'a str,
}

/// `send_resources` arguments.
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone, Default))]
pub struct SendedUnits {
    pub wood: u64,
    pub stone: u64,
    pub food: u64,
    pub magic: u64,
}

/// `send_resources` arguments.
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone, Default))]
pub struct SendResourcesArgs<'a> {
    // Destination Fortress.
    pub destination_fortress: &'a str,
    // Resources sended.
    pub sended_resources: SendedUnits,
    // Message.
    pub message: &'a str,
    // TODO: mabye add sender?
}
