use nalgebra::{DVector, Vector2};
use bno055::mint::{Quaternion,Vector3};
use bno055::{Bno055,Error as BNOError};
use embedded_hal::blocking::i2c::{WriteRead,Write};
use rppal::{
    hal::Delay,
    i2c::{Error as I2cError, I2c},
};

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

struct Bno055imu {
    imu: Bno055<I2c>,
}

impl Imu for Bno055imu
{
    fn reading(&self) -> Result<ImuReading, I2cError> {
        let accel = self.imu.linear_acceleration_fixed();
        let angaccel = self.

        ImuReading {
            accel,
            veloc: self.vel + accel,
            angacce
        }
    }
}

struct Bias {
    offset: Vector3<f32>,
    rotation: Quaternion<f32>,
}

// Remove this ASAP. Will will kill me if he sees this.
#[derive(Clone)]
struct ImuReading {
    accel: Vector3<f32>,
    veloc: Vector3<f32>,
    angaccel: Vector3<f32>,
    angveloc: Vector3<f32>,
}

struct VirtualImu {
    algorithm: DeadReckoningAlgorithms,
    // imus: Vec<imu> ,
    imus: Vec<(Box<dyn Imu>, Bias)>,
}


impl VirtualImu {
    fn new(imus: Vec<(Box<dyn Imu>, Bias)>) -> VirtualImu {
        VirtualImu {
            algorithm: DeadReckoningAlgorithms::default(),
            imus,
        }
    }

    fn from_one(imu: Box<dyn Imu>, bias: Bias) -> VirtualImu {
        VirtualImu {
            algorithm: DeadReckoningAlgorithms::default(),
            imus: vec![(imu, bias)],
        }
    }

    fn imu_integrate(&mut self) -> Option<ImuReading> {
        // No measurement model, just dead reckoning
        let mut measurements: Vec<ImuReading> = self.imus.iter().map(|x| x.reading()).collect();

        // Virtual IMU reading
        match &self.algorithm {
            DeadReckoningAlgorithms::RollingAverage => rolling_average(measurements),
            DeadReckoningAlgorithms::KalmanFilter => unimplemented!()
        }
    }

}

struct Robot {
    posit: Vector2<f32>,
    veloc: Vector2<f32>,
    accel: Vector2<f32>,
    angveloc: Vector2<f32>,
}

impl Robot {
    fn best_guess() {}
}


// Takes a Vec for convenience right now. Consider making it an array
// Assume that the period that the measurements are taken over are short
// Returns a virtual IMU reading
fn rolling_average(readings: Vec<ImuReading>) -> Option<ImuReading> {
    let mut init = DVector::zeros(2);
    let readingsnum = readings.len();

    if readingsnum == 0 { return None };

    readings.iter()
        .fold(&mut init, |a, x| {
            *a += DVector::from_vec(vec![x.veloc, x.angveloc]);
            a
        });

    init /= readingsnum as f32;

    Some(ImuReading {
        veloc: init[0],
        angveloc: init[1],
        accel: 0.0,
        angaccel: 0.0,
    })
}

fn main() {

}
