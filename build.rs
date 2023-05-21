use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    RuntimeBuilder::from_flexspi(Family::Imxrt1020, 16 * 1024 * 1024)
        .build()
        .unwrap();
}
