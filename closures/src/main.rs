use closures::{ generate_workout };

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_rand_num = 7;

  generate_workout(&simulated_user_specified_value, simulated_rand_num);
}
