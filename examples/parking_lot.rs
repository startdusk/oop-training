// Design a Parking Lot (设计一个停车场)
// 一个停车场是一栋楼
// 每层楼有几个停车位(停车位都是固定大小)
// 而汽车是

use std::collections::HashMap;

use chrono::Timelike;

/// Vehicle 车辆(车有不同的型号, 汽车, 自行车, 电单车)

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Vehicle {
    spot_size: i32, // 车辆尺寸(车位大小)
}

impl Vehicle {
    fn new(spot_size: i32) -> Self {
        Vehicle { spot_size }
    }

    fn get_spot_size(&self) -> i32 {
        self.spot_size
    }
}

pub struct Driver {
    id: i32,
    vehicle: Vehicle,
    payment_due: i32,
}

impl Driver {
    pub fn new(id: i32, vehicle: Vehicle) -> Self {
        Driver {
            id,
            vehicle,
            payment_due: 0,
        }
    }

    pub fn get_vehicle(&self) -> Vehicle {
        self.vehicle.clone()
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    // 计算停车费用
    pub fn charge(&mut self, amount: i32) {
        self.payment_due += amount;
    }
}

/// Car 普通汽车
pub struct Car {
    vehicle: Vehicle,
}

impl Car {
    pub fn new() -> Self {
        Car {
            vehicle: Vehicle::new(1),
        }
    }
}

/// Limo 豪华轿车
pub struct Limo {
    vehicle: Vehicle,
}

impl Limo {
    pub fn new() -> Self {
        Limo {
            vehicle: Vehicle::new(2),
        }
    }
}

/// SemiTruck 皮卡
pub struct SemiTruck {
    vehicle: Vehicle,
}

impl SemiTruck {
    pub fn new() -> Self {
        SemiTruck {
            vehicle: Vehicle::new(3),
        }
    }
}

/// ParkingFloor 停车的楼层
pub struct ParkingFloor {
    spots: Vec<i32>,
    vehicle_map: HashMap<Vehicle, (usize, usize)>,
}

impl ParkingFloor {
    pub fn new(spot_count: usize) -> Self {
        ParkingFloor {
            spots: vec![0; spot_count],
            vehicle_map: HashMap::new(),
        }
    }

    pub fn park_vehicle(&mut self, vehicle: Vehicle) -> bool {
        let size = vehicle.get_spot_size();
        let mut l = 0;
        let mut r = 0;
        while r < self.spots.len() {
            if self.spots[r] != 0 {
                l = r + 1;
            }
            if (r as i32 - l as i32 + 1) == size {
                // we found enough spots, park the vehicle
                for k in l..=r {
                    self.spots[k] = 1;
                }
                self.vehicle_map.insert(vehicle, (l, r));
                return true;
            }
            r += 1;
        }
        false
    }

    pub fn remove_vehicle(&mut self, vehicle: &Vehicle) {
        if let Some(start_end) = self.vehicle_map.get(vehicle) {
            let (start, end) = *start_end;
            for i in start..=end {
                self.spots[i] = 0;
            }
            self.vehicle_map.remove(vehicle);
        }
    }

    pub fn get_parking_spots(&self) -> &[i32] {
        &self.spots
    }

    pub fn get_vehicle_spots(&self, vehicle: &Vehicle) -> Option<&(usize, usize)> {
        self.vehicle_map.get(vehicle)
    }
}

/// ParkingGarage 停车场
pub struct ParkingGarage {
    parking_floors: Vec<ParkingFloor>,
}

impl ParkingGarage {
    pub fn new(floor_count: i32, spots_per_floor: i32) -> ParkingGarage {
        let mut parking_floors = Vec::new();
        for _ in 0..floor_count {
            parking_floors.push(ParkingFloor::new(spots_per_floor as usize));
        }
        ParkingGarage { parking_floors }
    }

    pub fn park_vehicle(&mut self, vehicle: Vehicle) -> bool {
        for floor in &mut self.parking_floors {
            if floor.park_vehicle(vehicle.clone()) {
                return true;
            }
        }
        false
    }

    pub fn remove_vehicle(&mut self, vehicle: &Vehicle) -> bool {
        for floor in &mut self.parking_floors {
            if floor.get_vehicle_spots(vehicle).is_some() {
                floor.remove_vehicle(vehicle);
                return true;
            }
        }
        false
    }
}

/// ParkingSystem 停车系统
pub struct ParkingSystem {
    parking_garage: ParkingGarage,

    // 每小时的停车费用
    hourly_rate: i32,
    // 记录每辆车的停车起始时间
    time_parked: HashMap<i32, i32>,
}

impl ParkingSystem {
    pub fn new(parking_garage: ParkingGarage, hourly_rate: i32) -> ParkingSystem {
        let time_parked = HashMap::new();
        ParkingSystem {
            parking_garage,
            hourly_rate,
            time_parked,
        }
    }

    pub fn park_vehicle(&mut self, driver: &Driver) -> bool {
        let current_hour = chrono::Local::now().hour();
        let is_parked = self.parking_garage.park_vehicle(driver.get_vehicle());
        if is_parked {
            self.time_parked
                .insert(driver.get_id(), current_hour as i32);
        }
        is_parked
    }

    pub fn remove_vehicle(&mut self, driver: &mut Driver) -> bool {
        if !self.time_parked.contains_key(&driver.get_id()) {
            return false;
        }
        let current_hour = chrono::Local::now().hour() as i32;
        let time_parked = current_hour - self.time_parked[&driver.get_id()];

        // 按车型计算停车费用
        driver.charge(driver.get_vehicle().get_spot_size() * time_parked * self.hourly_rate);

        self.time_parked.remove(&driver.get_id());
        self.parking_garage.remove_vehicle(&driver.get_vehicle())
    }
}

fn main() {
    // 停车场有3层楼, 每一楼有2个停车位
    let parking_garage = ParkingGarage::new(3, 2);
    // 一个停车系统, 每小时每个停车位停车费5块
    let mut parking_system = ParkingSystem::new(parking_garage, 5);

    let car = Car::new();
    let limo = Limo::new();
    let semi_truck = SemiTruck::new();
    // 这里写得不好，带后续有空修改
    let mut driver1 = Driver::new(1, car.vehicle);
    let mut driver2 = Driver::new(2, limo.vehicle);
    let mut driver3 = Driver::new(3, semi_truck.vehicle); // 皮卡需要3个停车位, 而每层楼只有两个停车位, 所以停不下

    println!(
        "普通汽车需要1个停车位, 而每层楼只有两个停车位, 所以停得下={}",
        parking_system.park_vehicle(&driver1)
    ); // true
    println!(
        "豪华轿车需要2个停车位, 而每层楼只有两个停车位, 所以停得下={}",
        parking_system.park_vehicle(&driver2)
    ); // true
    println!(
        "皮卡需要3个停车位, 而每层楼只有两个停车位, 所以停不下={}",
        parking_system.park_vehicle(&driver3)
    ); // false

    println!("{}", parking_system.remove_vehicle(&mut driver1)); // true
    println!("{}", parking_system.remove_vehicle(&mut driver2)); // true
    println!("{}", parking_system.remove_vehicle(&mut driver3)); // false
}
