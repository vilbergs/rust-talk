use rand::Rng;

let mut memo = HashMap::new();

pub fn fake_data() -> Vec<u32> {
  let mut rng = rand::thread_rng();

  let mut vec = Vec::new();

  for _ in 0..100000 {
    vec.push(rng.gen_range(20..1001))
  }

  vec
}
