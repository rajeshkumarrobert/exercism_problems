// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Debug)]
pub struct Duration{
    pub age: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let earth_year_in_seconds = 31557600;
        let mut age = s as f64/earth_year_in_seconds as f64;
        age = (age * 100.0).round()/100.0;
        Duration { age: age }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let mercury_orbital_period = 0.2408467;
        let mut res = d.age/mercury_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let venus_orbital_period = 0.61519726;
        let mut res = d.age/venus_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let earth_orbital_period = 1.0;
        let mut res = d.age/earth_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let mars_orbital_period = 1.8808158;
        let mut res = d.age/mars_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let jupiter_orbital_period = 11.862615;
        let mut res = d.age/jupiter_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let saturn_orbital_period = 29.447498;
        let mut res = d.age/saturn_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let uranus_orbital_period = 84.016846;
        let mut res = d.age/uranus_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let neptune_orbital_period = 164.79132;
        let mut res = d.age/neptune_orbital_period;
        res = (res * 100.0).round()/100.0;
        res
    }
}
