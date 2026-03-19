use rand::Rng;

fn sample_l(m_l: f64) -> u64 {
    let mut rng = rand::thread_rng();

    // Generate uniform random number in (0,1)
    let u: f64 = rng.gen_range(0.0..1.0);

    // Apply transformation
    let value = -u.ln() * m_l;

    // Floor and convert to integer
    value.floor() as u64
}