use frame_support::pallet_macros::pallet_section;

/// Define all events used in the pallet.
#[pallet_section]
mod storages {
    // use frame_support::pallet_prelude::*;
    // #[pallet::pallet]
    // pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Kitties2<T> = StorageMap<_, Blake2_128Concat, u32, Kitty>;
}
