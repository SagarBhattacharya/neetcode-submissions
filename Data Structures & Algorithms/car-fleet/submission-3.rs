impl Solution {
  pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut lane = position.iter().copied()
      .zip(speed.iter().copied())
      .collect::<Vec<(i32, i32)>>();
    lane.sort();

    let mut fleet_count = 0;
    let mut last_fleet_time = 0.0;

    for &(p, s) in lane.iter().rev() {
      let t = (target - p) as f32 / s as f32;
      if t > last_fleet_time {
        fleet_count += 1;
        last_fleet_time = t;
      }
    }
    fleet_count
  }
}