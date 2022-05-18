use rand::Rng;

pub fn fake_data() -> Vec<u32> {
  let mut rng = rand::thread_rng();

  let mut vec = Vec::new();

  for _ in 1..1001 {
    vec.push(rng.gen_range(20..1001))
  }

  vec
}