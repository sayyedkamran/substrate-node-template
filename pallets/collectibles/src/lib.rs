#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, Randomness};

    pub enum Color {
        Red,
        Yellow,
        Blue,
        Green
    }

    type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub struct Collectible<T: Config> {
        // Unsigned integers of 16 bytes to represent a unique identifier
        pub unique_id: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub color: Color,
        pub owner: T::AccountId,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId>;
        type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumberFor<T>>;

    #[pallet::constant]
    type MaximumOwned: Get<u32>;
}
}


