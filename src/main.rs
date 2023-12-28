use battery::{
    units::{ElectricPotential, Energy, Power},
    Manager, State,
};
use chrono::prelude::*;
use csv::Writer;
use std::{
    error::Error,
    fs::{File, OpenOptions},
};
use tokio::time::{self, Duration};

const LOGGING_INTERVAL: u64 = 30; //in sec

#[tokio::main]
async fn main() -> Result<(), battery::Error> {
    let mut interval = time::interval(Duration::from_secs(LOGGING_INTERVAL));
    let manager = Manager::new()?;
    // Replace `func()` with the appropriate function call or remove the line if it is not needed.
    //  {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("battery_log.csv")
        .unwrap();

    loop {
        for (idx, maybe_battery) in manager.batteries()?.enumerate() {
            let battery = maybe_battery?;

            if let Err(_err) = csv_log(
                file.try_clone().unwrap(),
                idx,
                battery.state(),
                battery.voltage(),
                battery.energy(),
                battery.energy_rate(),
            ) {}
        }
        interval.tick().await;
    }
}

fn csv_log(
    file: File,
    idx: usize,
    state: State,
    voltage: ElectricPotential,
    energy: Energy,
    energy_rate: Power,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(file);
    println!(
        "Logging:: {},{},{},{},{},{}",
        Utc::now().to_string(),
        idx.to_string(),
        state.to_string(),
        format!("{:?}", voltage),
        format!("{:?}", energy),
        format!("{:?}", energy_rate)
    );
    wtr.write_record(&[
        Utc::now().to_string(),
        idx.to_string(),
        state.to_string(),
        format!("{:?}", voltage.value),
        format!("{:?}", energy.value),
        format!("{:?}", energy_rate.value),
    ])?;
    wtr.flush()?;
    Ok(())
}
