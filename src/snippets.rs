// just used for reference. Handy video here for learning more:
// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=2s

use rand::Rng;

fn random_number() {
    // example: this will generate a random value from 1-100
    let rn = rand::thread_rng().gen_range(1..101);
}