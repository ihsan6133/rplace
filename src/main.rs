use std::{error::Error, time::{Instant, Duration}, path::Path};

use reqwest::{Client, header::{RANGE, HeaderValue}};
use tokio::{fs::File, io::AsyncWriteExt};
use indicatif::{ProgressBar, ProgressStyle};


struct RangeIter {
    start: usize,
    end: usize,
    chunk_size: usize,
    pb: ProgressBar,
    start_time: Instant
}

impl RangeIter {
    pub fn new(start: usize, end: usize, chunk_size: usize) -> Self {
        let pb = ProgressBar::new(end as u64 - 1);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
            .progress_chars("##-"));
        pb.set_message("Downloading");

        RangeIter { start, end, chunk_size, pb, start_time: Instant::now() }
    }
}

impl Iterator for RangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            self.pb.set_position((self.end - 1) as u64);
            None
            
        } else {

            
            let elapsed_time = self.start_time.elapsed();
            
            const TARGET_CHUNK_TIME: Duration = Duration::from_secs(1);
            
            let ratio = TARGET_CHUNK_TIME.as_secs_f32() / elapsed_time.as_secs_f32();
            self.chunk_size = std::cmp::min((self.chunk_size as f32 * ratio) as usize, 10000000);

            let prev_start = self.start;
            self.start += std::cmp::min(self.chunk_size, self.end - self.start + 1);
            
            self.pb.set_position(self.start as u64);
            
            self.start_time = Instant::now();
            
            Some(HeaderValue::from_str(format!("bytes={}-{}", prev_start, self.start - 1).as_str()).unwrap())
            
        }
    }
}

trait ErrStr<T> {
    fn err_str(self) -> Result<T, String>;
}

impl<T, E: Error> ErrStr<T> for Result<T, E> {
    fn err_str(self) -> Result<T, String> {
        self.map_err(|e|e.to_string())
    }
}
async fn download_file(url: &str, file_path: &str) -> Result<(), String> {
    let client = Client::builder().gzip(true).build().expect("Failed to create reqwest client");
    let mut file = File::create(file_path).await.err_str()?;
    
    let res = client.get(url).send().await.err_str()?;
    let length = res.content_length().expect("expected content length!");

    

    const CHUNK_SIZE: usize = 10000;

    for range in RangeIter::new(0, (length - 1) as usize, CHUNK_SIZE) {

        let response = client.get(url).header(RANGE, range).send().await.err_str()?;
        let bytes = response.bytes().await.err_str()?;
        file.write_all(&bytes).await.err_str()?;
    }

    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    

    for i in 8..=51 {
        let url = std::format!("https://placedata.reddit.com/data/canvas-history/2023/2023_place_canvas_history-{i:0>12}.csv.gzip");
        let file_path = std::format!("dataset/2023_place_canvas_history-{i:0>12}.csv.gz");
        
        download_file(&url, &file_path).await?;
    }

    Ok(())
}
 