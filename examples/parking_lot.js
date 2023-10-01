class Vehicle {
    #spotSize;

    constructor(spotSize) {
        this.#spotSize = spotSize;
    }

    getSpotSize() {
        return this.#spotSize;
    }
}

class Driver {
    #id;
    #vehicle;
    #paymentDue;

    constructor(id, vehicle) {
        this.#id = id;
        this.#vehicle = vehicle;
        this.#paymentDue = 0;
    }

    getVehicle() {
        return this.#vehicle;
    }

    getId() {
        return this.#id;
    }

    charge(amount) {
        this.#paymentDue += amount;
    }
}

class Car extends Vehicle {
    constructor() {
        super(1);
    }
}

class Limo extends Vehicle {
    constructor() {
        super(2);
    }
}

class SemiTruck extends Vehicle {
    constructor() {
        super(3);
    }
}

class ParkingFloor {
    #spots;
    #vehicleMap;

    constructor(spotCount) {
        this.#spots = Array(spotCount).fill(0);
        this.#vehicleMap = {};
    }

    parkVehicle(vehicle) {
        const size = vehicle.getSpotSize();
        let l = 0, r = 0;
        while (r < this.#spots.length) {
            if (this.#spots[r] !== 0) {
                l = r + 1;
            }
            if (r - l + 1 === size) {
                // we found enough spots, park the vehicle
                for (let k = l; k <= r; k++) {
                    this.#spots[k] = 1;
                }
                this.#vehicleMap[vehicle] = [l, r];
                return true;
            }
            r++;
        }
        return false;
    }

    removeVehicle(vehicle) {
        const [start, end] = this.#vehicleMap[vehicle];
        for (let i = start; i <= end; i++) {
            this.#spots[i] = 0;
        }
        delete this.#vehicleMap[vehicle];
    }

    getParkingSpots() {
        return this.#spots;
    }

    getVehicleSpots(vehicle) {
        return this.#vehicleMap[vehicle];
    }
}

class ParkingGarage {
    #parkingFloors;

    constructor(floorCount, spotsPerFloor) {
        this.#parkingFloors =
            Array(floorCount).fill().map(p => new ParkingFloor(spotsPerFloor));
    }

    parkVehicle(vehicle) {
        for (const floor of this.#parkingFloors) {
            if (floor.parkVehicle(vehicle)) {
                return true;
            }
        }
        return false;
    }

    removeVehicle(vehicle) {
        for (const floor of this.#parkingFloors) {
            if (floor.getVehicleSpots(vehicle)) {
                floor.removeVehicle(vehicle);
                return true;
            }
        }
        return false;
    }
}

class ParkingSystem {
    #parkingGarage;
    #hourlyRate;
    #timeParked;    // map driverId to time that they parked

    constructor(parkingGarage, hourlyRate) {
        this.#parkingGarage = parkingGarage;
        this.#hourlyRate = hourlyRate;
        this.#timeParked = {};
    }

    parkVehicle(driver) {
        const currentHour = new Date().getHours();
        const isParked = this.#parkingGarage.parkVehicle(driver.getVehicle());
        if (isParked) {
            this.#timeParked[driver.getId()] = currentHour;
        }
        return isParked;
    }

    removeVehicle(driver) {
        if (!this.#timeParked.hasOwnProperty(driver.getId())) {
            return false;
        }
        const currentHour = new Date().getHours();
        const timeParked = Math.ceil(currentHour - this.#timeParked[driver.getId()]);
        driver.charge(timeParked * this.#hourlyRate);

        delete this.#timeParked[driver.getId()];
        return this.#parkingGarage.removeVehicle(driver.getVehicle());
    }
}

const parkingGarage = new ParkingGarage(3, 2);
const parkingSystem = new ParkingSystem(parkingGarage, 5);

const driver1 = new Driver(1, new Car());
const driver2 = new Driver(2, new Limo());
const driver3 = new Driver(3, new SemiTruck());

console.log(parkingSystem.parkVehicle(driver1));    // true
console.log(parkingSystem.parkVehicle(driver2));    // true
console.log(parkingSystem.parkVehicle(driver3));    // false

console.log(parkingSystem.removeVehicle(driver1));  // true
console.log(parkingSystem.removeVehicle(driver2));  // true
console.log(parkingSystem.removeVehicle(driver3));  // false
