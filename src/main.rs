
mod memory;
use anyhow::Result;
use std::time::Duration;
use memory::SOAMemoryMap;
use indicatif::ProgressBar;

const KILL_COUNT: i32 = 2500;

#[tokio::main]
async fn main() -> Result<()> {

    println!("Skies of Arcadia Kill Counter");
    let mut completed = false;
    let progressbar = ProgressBar::new(KILL_COUNT as u64);
    loop {
        println!("Waiting for Dolphin...");
        let memory = SOAMemoryMap::wait_for_dolphin();

        while memory.is_running() {
            
            if let Ok(count) = memory.read_enemies_killed() {
                
                if !progressbar.is_finished() {
                    progressbar.set_position(count as u64);
                }
                
                if !completed && count >= KILL_COUNT {
                    completed = true;
                    progressbar.finish();
                    println!();
                    println!("Grind Completed \u{1F389}\u{1F389}\u{1F389}");
                }

                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }

        println!("Dolphin Disconnected...")
    }
}
