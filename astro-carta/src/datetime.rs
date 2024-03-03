mod month;
pub mod timedelta;

pub use timedelta::TimeDelta;

pub struct DateTime {
    duration: TimeDelta,
}
