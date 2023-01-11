use yahoo_finance::{history, Interval, Timestamped};

// retrieve 6 months worth of data
#[tokio::main]
async fn main() {
   let data = history::retrieve_interval("AAPL", Interval::_6mo).await.unwrap();

   // print out some high numbers!
   for bar in &data {
      println!("Apple hit an intraday high of ${:.2} on {}.", bar.high, bar.datetime().format("%b %e %Y"));
   }
}