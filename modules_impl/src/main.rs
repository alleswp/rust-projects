mod sample_module;
use sample_module::SampleImpl;

fn main() {
    let mySampleImpl = SampleImpl::new("Wayne".to_string());
    mySampleImpl.hello_world();
}