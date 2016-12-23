pub struct SampleImpl {
    name: String
}

impl SampleImpl {
    pub fn new(newName: String) -> SampleImpl {
        SampleImpl{ name: newName }
    }
    pub fn hello_world(&self) {
        println!("My Name is: {:?}", self.name)
    }
}