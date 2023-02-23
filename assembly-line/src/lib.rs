pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total_production = f64::from(speed) * 221.0;
    
    if speed < 5 {
        return total_production;
    }
    if speed < 9 {
        return total_production * 0.9;
    }
    return total_production * 0.77;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let hour_rate = production_rate_per_hour(speed);

    return hour_rate as u32 / 60;
}
