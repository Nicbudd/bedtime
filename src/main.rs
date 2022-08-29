// Linux dependencies: top 
use std::process::Command;
use regex::Regex;
use rand::prelude::*;
use std::{thread, time::{self, Duration}};

const ITAL: &str = "\x1b[3m";
const CLEAR: &str = "\x1b[0m";

const MIN_SLEEP_TIME: u64 = (7 * 60 * 60) + (30 * 60);
// const MIN_SLEEP_TIME: u64 = 120;

const FALL_ASLEEP_MAX_CPU: f32 = 25.0;
const STAY_ASLEEP_MAX_CPU: f32 = 50.0;

#[allow(unused)]
const ONE_SEC: Duration = Duration::from_secs(1);
const TWO_SEC: Duration = Duration::from_secs(2);
const THREE_SEC: Duration = Duration::from_secs(5);
const FIVE_SEC: Duration = Duration::from_secs(5);
const TEN_SEC: Duration = Duration::from_secs(10);

const MIN_SILENT_TIME: u64 = 60 * 10; // 10 minutes
const MAX_SILENT_TIME: u64 = 60 * 60; // 60 minutes
// const MIN_SILENT_TIME: u64 = 5;
// const MAX_SILENT_TIME: u64 = 6;

const MIN_NOISE_COUNT: u64 = 5;
const MAX_NOISE_COUNT: u64 = 15;

const MIN_HONK_O: usize = 1;
const MAX_HONK_O: usize = 4;

const MIN_SHNOO_O: usize = 2;
const MAX_SHNOO_O: usize = 10;

const MIN_SNORE_O: usize = 1;
const MAX_SNORE_O: usize = 10;

const MIN_MIMIMI: usize = 3;
const MAX_MIMIMI: usize = 15;

const MIN_Z: usize = 3;



#[derive(Debug, Copy, Clone)]
enum State {
    StartOfSleep, EndOfSleep,
    Silent,
    Awoken, 
    HonkShnoo, HonkMimi, ZzZ, Snore,
}



impl State {

    fn run(&self, sleeptime: &mut u64, rng: &mut ThreadRng) -> State {
        match self {

            State::StartOfSleep => {

                if cpu() > FALL_ASLEEP_MAX_CPU {
                    println!("Gaahhh! I can't go to sleep with how noisy it is right now! Quiet down, please!");
                    
                    thread::sleep(FIVE_SEC);

                    while cpu() > FALL_ASLEEP_MAX_CPU {
                        thread::sleep(FIVE_SEC);
                    }
                }

                println!("Thanks for the rest, I desprately needed it!");
                thread::sleep(TWO_SEC);
                println!("{}Your terminal pulls a blanket over itself and rests its head on a pillow, slowly drifting into a deep sleep. You have decided to being careful in what you do, being quiet so that you don't wake it up.{}", ITAL, CLEAR);
                
                *sleeptime = 0;

                State::Silent
                
            } 

            State::EndOfSleep => {
                
                let hours = *sleeptime / 60 / 60;
                let minutes = (*sleeptime / 60) % 60;

                println!("{}Your terminal awakens from its slumber, feeling a bit groggy, but much more equipped to tackle the rest of the day.{}", ITAL, CLEAR);
                thread::sleep(FIVE_SEC);
                println!("Mmmmmmm, good morning. Wow, I really needed that sleep. How long did I sleep? {} hours and {} minutes? Splendid. Thanks for giving me that opportunity to sleep! Now, if only I had some coffee...", hours, minutes);
                thread::sleep(FIVE_SEC);

                std::process::exit(0);
            }

            State::Silent => {

                let segment_time = rng.gen_range(MIN_SILENT_TIME..MAX_SILENT_TIME);
                let duration_segment_time = Duration::from_secs(segment_time);
                let now = time::Instant::now();

                let mut ten_sec_counter = 0;

                while now.elapsed() < duration_segment_time {
                    if ten_sec_counter % 5 == 0 {
                        println!("\n");
                    }
                    
                    thread::sleep(TEN_SEC);

                    ten_sec_counter += 1;

                    if cpu() > STAY_ASLEEP_MAX_CPU {
                        return State::Awoken;
                    }
                }

                *sleeptime += now.elapsed().as_secs();

                if *sleeptime > MIN_SLEEP_TIME {
                    State::EndOfSleep
                } else {
                    let noisy_sleep = [State::HonkShnoo, State::HonkMimi, State::ZzZ, State::ZzZ, State::Snore];
                    *noisy_sleep.choose(rng).unwrap()
                }
            }

            State::Awoken => {
                *sleeptime = 0;

                println!("..huhhhh");
                thread::sleep(TWO_SEC);
                println!("...ahhhhh wha");
                thread::sleep(TWO_SEC);
                println!("what are you doing! Stop making such a ruckus! You're keeping me awake!");
                thread::sleep(FIVE_SEC);

                while cpu() > FALL_ASLEEP_MAX_CPU {
                    thread::sleep(TEN_SEC);
                }

                println!("Ughhh. Thank you! Now I have to start all over again. What a waste of time...");
                println!("{}Your terminal frustratedly fluffs up its pillow again and goes back to sleep.{}", ITAL, CLEAR);

                State::Silent
            }

            State::HonkShnoo => {

                let now = time::Instant::now();

                for _i in 0..rng.gen_range(MIN_NOISE_COUNT..MAX_NOISE_COUNT) {

                    let honk_o = rng.gen_range(MIN_HONK_O..=MAX_HONK_O);
                    let shnoo_o = rng.gen_range(MIN_SHNOO_O..=MAX_SHNOO_O);

                    print!("h");
                    let o = "o".repeat(honk_o);
                    print!("{}", o);
                    print!("nk");
                    print!("\n");

                    thread::sleep(TWO_SEC);

                    print!("shn");
                    let o = "o".repeat(shnoo_o);
                    print!("{}", o);
                    print!("\n");

                    thread::sleep(THREE_SEC);

                }

                *sleeptime += now.elapsed().as_secs();

                State::Silent
            }

            State::HonkMimi => {

                let now = time::Instant::now();

                for _i in 0..rng.gen_range(MIN_NOISE_COUNT..MAX_NOISE_COUNT) {
                    
                    let honk_o = rng.gen_range(MIN_HONK_O..=MAX_HONK_O);
                    let mimi = rng.gen_range(MIN_MIMIMI..=MAX_MIMIMI);

                    print!("h");
                    let o = "o".repeat(honk_o);
                    print!("{}", o);
                    print!("nk");
                    print!("\n");

                    thread::sleep(TWO_SEC);

                    let mi = "mi".repeat(mimi);
                    print!("{}", mi);
                    print!("\n");

                    thread::sleep(THREE_SEC);

                }

                *sleeptime += now.elapsed().as_secs();

                State::Silent
            }

            State::ZzZ => {

                let now = time::Instant::now();
                
                for _i in 0..rng.gen_range(MIN_NOISE_COUNT..MAX_NOISE_COUNT) {
                    
                    let mut z_msg = String::new();

                    while z_msg.chars().count() < MIN_Z {
                        z_msg = String::new();

                        loop {
                            let c = match rng.gen_range(0..10) {
                                0..=3 => 'Z',
                                4..=8 => 'z',
                                9 => break,
                                _ => break
                            };
    
                            z_msg.push(c);
                            
                        }
                    }

                    println!("{}", z_msg);

                    thread::sleep(FIVE_SEC);

                }

                *sleeptime += now.elapsed().as_secs();

                State::Silent
            }

            State::Snore => {

                let now = time::Instant::now();

                for _i in 0..rng.gen_range(MIN_NOISE_COUNT..MAX_NOISE_COUNT) {
                    
                    let snore_o = rng.gen_range(MIN_SNORE_O..=MAX_SNORE_O);

                    print!("{}sn", ITAL);
                    let o = "o".repeat(snore_o);
                    print!("{}", o);
                    print!("re{}", CLEAR);
                    print!("\n");

                    thread::sleep(FIVE_SEC);

                }

                *sleeptime += now.elapsed().as_secs();

                State::Silent
            }
        }
    }

}



fn cpu() -> f32 {
    let top = Command::new("top")
                      .arg("-bn1")
                      .output()
                      .expect("top command not working.");
    
    let outStr = std::str::from_utf8(&top.stdout).unwrap(); 
    // println!("{}", outStr);
    let RE = Regex::new(r"%Cpu\(s\).*, ([0-9\.]+)%* id.*").unwrap();
    let cap = RE.captures(outStr).unwrap();
    let value = 100.0 - cap[1].parse::<f32>().unwrap();
    
    value
} 

fn main() {
    let mut rng = rand::thread_rng();
    let mut state = State::StartOfSleep;
    // let mut state = State::HonkMimi;
    let mut total_sleeptime = 0;

    loop {
        // dbg!(state);
        // dbg!(total_sleeptime);
        
        state = state.run(&mut total_sleeptime, &mut rng);

    }
    
    
}