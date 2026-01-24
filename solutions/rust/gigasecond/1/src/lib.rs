use time::PrimitiveDateTime as DateTime;
use time::Duration;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_seconds = Duration::seconds(1000000000);
    start.checked_add(giga_seconds).unwrap()
}
