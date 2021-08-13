
pub fn average(averages: &Vec<f64>) -> f64 {
    let length = averages.len();
    let addend = averages.into_iter().fold(0., |x, y| x + y);
    let result = addend / length as f64;
    result
}