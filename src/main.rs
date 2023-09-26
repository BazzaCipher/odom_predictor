use nalgebra::{DVector, Vector2};
use bno055::mint::{Quaternion,Vector3};
use bno055::{Bno055,Error as BNOError};

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
        ImuReading {
            accel: 0.3,
            veloc: 5.0,
            angaccel: 2.0,
            angveloc: 1.0,
        }
    }
}

struct Bno055imu<I: WriteRead<Error = BNOError> + Write<Error = BNOError>> {
    imu: Bno055<I>,
    offset: Vector3<f32>,
    rotation: Quaternion<f32>,
}

// Output from IMU is likely mint::Vector3
type Pose = Vector2<f32>;

// Remove this ASAP. Will will kill me if he sees this.
#[derive(Clone)]
struct ImuReading {
    accel: f32,
    veloc: f32,
    angaccel: f32,
    angveloc: f32,
}

struct Odom {
    algorithm: DeadReckoningAlgorithms,
    // imus: Vec<imu> ,
    imus: Vec<Box<dyn Imu>>,
}


impl Odom {
    fn new(imus: Vec<Box<dyn Imu>>) -> Odom {
        Odom {
            algorithm: DeadReckoningAlgorithms::default(),
            imus,
        }
    }

    fn imu_integrate(&mut self) {
        // No measurement model, just dead reckoning
        let mut measurements: Vec<ImuReading> = self.imus.iter().map(|x| x.reading()).collect();

        // Virtual IMU reading
        let vimu = match &self.algorithm {
            DeadReckoningAlgorithms::RollingAverage => rolling_average(measurements),
            DeadReckoningAlgorithms::KalmanFilter => unimplemented!()
        };
    }

}

// Takes a Vec for convenience right now. Consider making it an array
// Assume that the period that the measurements are taken over are short
// Returns a virtual IMU reading
fn rolling_average(readings: Vec<ImuReading>) -> Option<ImuReading> {
    let mut init = DVector::zeros(2);
    let readingsnum = readings.len() as f32;

    if readingsnum == 0. { return None };

    readings.iter()
        .fold(&mut init, |a, x| {
            *a += DVector::from_vec(vec![x.veloc, x.angveloc]);
            a
        });

    init /= readingsnum;

    Some(ImuReading {
        veloc: init[0],
        angveloc: init[1],
        accel: 0.0,
        angaccel: 0.0,
    })
}

fn main() {

}
