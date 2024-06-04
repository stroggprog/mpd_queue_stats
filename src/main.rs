extern crate mpd;

use mpd::Client;

const HOST: &str = "localhost:6600";

fn secs_to_time(t: u64) -> String {
    let seconds = t % 60;
    let minutes = (t / 60) % 60;
    let hours = (t / 60) / 60;
    if hours > 0 {
        format!("{:>02}:{:>02}:{:>02}", hours, minutes, seconds)
    }
    else {
        format!("{:>02}:{:>02}", minutes, seconds)
    }
}

fn main(){
    let mut conn = Client::connect(HOST).unwrap();
    let mut duration: u64 = 0; // u64 because that's what duration unwraps to
    let s = conn.status().unwrap();
    let lok: bool;
    let _state = match s.state {
        mpd::State::Stop => lok = false,
        mpd::State::Play => lok = true,
        mpd::State::Pause => lok = true,
    };
    // only get the rest of the information if there's information to fetch!
    if lok {
        let pos: u32 = s.song.unwrap().pos;
        let p = conn.queue().unwrap();
        let mut i: u32 = 0;
        let mut elapsed: u64 = s.time.unwrap().0.as_secs();
        for item in p {
            let time = item.duration.unwrap().as_secs();
            duration += time;
            if i < pos {
                elapsed += time;
            }
            i += 1;
        }
        let dur: i64 = duration as i64;
        let ele: i64 = elapsed as i64;
        let rt = dur.abs_diff(ele);
        let pc = ((elapsed as f32/duration as f32)*100.0) as f32;
        // ${voffset -103}
        println!("{}${{alignc}}{}${{alignr}}${{offset -5}}{}\n$alignc${{voffset -40}}${{offset 30}}{:.2}%", secs_to_time(elapsed as u64), secs_to_time(duration), secs_to_time(rt), pc);
    }
}
