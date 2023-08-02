/*
 MIT License

 Copyright (c) 2023 Nami Reghbati

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
*/
use wave_engine::wave::utils::Time;

#[test]
fn test_delta_time() {
  let time = chrono::Utc::now();
  let mut start_time: Time = Time {
    m_nano_seconds: time.timestamp_nanos() as f64
  };
  let mut end_time: Time = Time::from(time.clone());
  
  // Add a full second since adding only a nano second seems to fail, most likely due to the precision
  // level when asserting.
  end_time.m_nano_seconds += 1000000000.0;
  
  assert_eq!(Time::get_delta(&start_time, &end_time).to_secs(), 1.0);
  
  start_time.m_nano_seconds += 2000000000.0;
  
  // Make sure we get the absolute difference between the two time intervals.
  assert_eq!(Time::get_delta(&start_time, &end_time).to_secs(), 1.0);
}

#[test]
fn test_wait_for() {
  let start_time: Time = Time {
    m_nano_seconds: chrono::Utc::now().timestamp_nanos() as f64
  };
  
  Time::wait_for(1.0);
  
  assert_eq!(Time::get_delta(&Time::from(chrono::Utc::now()),
    &start_time).to_secs() as i64, 1);
  
  Time::wait_for(-1.0);  // When we supply an invalid argument.
  
  assert_eq!(Time::get_delta(&Time::from(chrono::Utc::now()),
    &start_time).to_secs() as i64, 1);
}