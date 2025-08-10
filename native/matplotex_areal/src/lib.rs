use rustler::{Env, Term, Encoder, NifResult};
use rustler::types::list::ListIterator;

#[derive(Debug)]
struct Statistics{
    min: i64,
    max: i64,
    intervals: [u64; 10]
}

impl Encoder for Statistics {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (
            rustler::types::atom::ok(),
            self.min, 
            self.max,
            self.intervals.to_vec()
        ).encode(env)
    }
}
#[rustler::nif]
fn compute_statistics<'a>(env: Env<'a>, numbers: Term<'a>) -> NifResult<Term<'a>>{
    let iter: ListIterator = numbers.decode()?;
    let nums: Vec<i64> = iter.map(|term| term.decode::<i64>()).collect::<Result<Vec<i64>, _>>()?;
    if nums.is_empty() {
        return Ok((
            rustler::types::atom::error(),
            "empty list"
        ).encode(env))
    }
    // Compute mins and max
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    // Compute intervals
    let mut intervals = [0u64; 10];
    let range = max - min;
    let bin_size = range / 10;
    for &num in &nums {
        if num >= min && num <= max {
            let bin = ((num as f64 - min as f64) / bin_size as f64).floor() as usize;
            let bin = bin.min(9);
            intervals[bin] += 1;
        }
    }
    println!("Hooraah Min: {}, Max: {}, Intervals: {:?}", min, max, intervals);
    let stats = Statistics {
        min,
        max,
        intervals
    };
    Ok(stats.encode(env))

}

rustler::init!("Elixir.Matplotex.Nif", [compute_statistics]);
