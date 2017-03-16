// normally holds Pwm drivers from
// https://github.com/rust-embedded/rust-sysfs-pwm the functions below normally
// interact with these but have been mocked out for testing.
pub struct Rover {}

impl Rover {
    pub fn new() -> Result<Rover, String> {
        // normally sets up anything that requires it
        println!("setting up");
        Ok(Rover {})
    }

    pub fn stop(&self) -> Result<(), String> {
        println!("stopping");
        Ok(())
    }

    pub fn set_speed(&self, left: i8, right: i8) -> Result<(), String> {
        println!("setting speed {} {}", left, right);
        Ok(())
    }

    pub fn get_speed(&self) -> Result<(i8, i8), String> {
        println!("getting speed");
        Ok((100, 100))
    }

    // normally tears down the rover to ensure it stops and cleans up the
    // system.
    pub fn unexport(self) -> Result<(), String> {
        println!("tearing down");
        Ok(())
    }
}
