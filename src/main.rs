use nalgebra::{Vector2};
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};

const IMU_COUNT: usize = 3;
const QUEUE_LENGTH: usize = 5;

#[derive(Default)]
enum DeadReckoningAlgorithms {
    #[default]
    RollingAverage,
    KalmanFilter,
}

trait Imu {
    fn reading(&self) -> ImuReading;
}

struct BadImu {

}

impl Imu for BadImu {
    fn reading(&self) -> ImuReading {
        ImuReading {}
    }
}

type Pose = Vector2<f32>;

// Remove this ASAP. Will will kill me if he sees this.
#[derive(Clone)]
struct ImuReading {
    a: acceleration
}

struct Odom {
    algorithm: DeadReckoningAlgorithms,
    // imus: Vec<imu> ,
    imu: Box<dyn Imu>,
    last_readings: ConstGenericRingBuffer<ImuReading, QUEUE_LENGTH>,
}


impl Odom {
    fn new() -> Odom {
        Odom {
            algorithm: DeadReckoningAlgorithms::default(),
            imu: Box::new(BadImu {}),
            last_readings: ConstGenericRingBuffer::default(),
        }
    }

    fn imu_integrate(&mut self, reading: &ImuReading) {
        // No measurement model, just dead reckoning
        let measurements: Vec<ImuReading> = self.last_readings.to_vec();
        match &self.algorithm {
            DeadReckoningAlgorithms::RollingAverage => rolling_average(measurements, reading),
            DeadReckoningAlgorithms::KalmanFilter => unimplemented!()
        };
    }

    fn update(&mut self) {
        self.imu_integrate(&self.imu.reading())
    }

}

// Takes a Vec for convenience right now. Consider making it an array
// Assume that the period that the measurements are taken over are short
fn rolling_average(readings: Vec<ImuReading>, last: &ImuReading) -> Pose {
    let sum = 0;
    for i in readings.len()
}

fn main() {

}
