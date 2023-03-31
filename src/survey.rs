use std::fmt;
use crate::attributes::{Attribute, SurveyInformationAttributes};
use netlink_rust as netlink;
use netlink_rust::generic;
use netlink_rust::Result;

pub struct SurveyResult {
  pub frequency: u32,
  pub frequency_offset: u32,
  pub channel: u32,
  pub noise: i8,
  pub time: u32,
  pub time_busy: u32,
  pub time_rx: u32,
  pub time_tx: u32,
  pub time_bss_rx: u32,
  pub time_scan: u32,
}

impl fmt::Display for SurveyResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
          f,
          "Freq:{}Mhz Offset:{}Mhz CH:{} Noise:{}dBm Busy Time:{}ms of {}ms",
          self.frequency,
          self.frequency_offset,
          self.channel,
          self.noise,
          self.time_busy,
          self.time
      )
  }
}

impl SurveyResult {
  fn freq_to_channel(mhz: u32) -> u32 {
    if mhz >= 2412 && mhz <= 2472 {
        return (mhz - 2407) / 5;
    }
    if mhz == 2484 {
        return 14;
    }
    if mhz >= 5000 && mhz < 6000 {
        return (mhz - 5000) / 5;
    } else {
        return 0xffffffff;
    }
}

  fn from_attributes(attributes: Vec<netlink::Attribute>) -> Result<SurveyResult> {
      let mut frequency = 0u32;
      let mut frequency_offset = 0u32;
      let mut channel = 0u32;
      let mut noise = 0i8;
      let mut time = 0u32;
      let mut time_busy = 0u32;
      let mut time_rx = 0u32;
      let mut time_tx = 0u32;
      let mut time_bss_rx = 0u32;
      let mut time_scan = 0u32;
      for attribute in attributes {
        let id = SurveyInformationAttributes::from(attribute.identifier);
        match id {
            SurveyInformationAttributes::Frequency => {
              frequency = attribute.as_u32()?;
              channel = Self::freq_to_channel(frequency);
            }
            SurveyInformationAttributes::Noise => {
              noise = attribute.as_i8()?;
            }
            SurveyInformationAttributes::Time => {
                time = attribute.as_u32()?;
            }
            SurveyInformationAttributes::TimeBusy => {
                time_busy = attribute.as_u32()?;
            }
            SurveyInformationAttributes::TimeRx => {
                time_rx = attribute.as_u32()?;
            }
            SurveyInformationAttributes::TimeTx => {
                time_tx = attribute.as_u32()?;
            }
            SurveyInformationAttributes::TimeScan => {
                time_scan = attribute.as_u32()?;
            }
            SurveyInformationAttributes::BssRx => {
                time_bss_rx = attribute.as_u32()?;
            }
            SurveyInformationAttributes::FrequencyOffset => {
                frequency_offset = attribute.as_u32()?;
            }
            _ => (),
        }
    }
      Ok(SurveyResult {
          frequency,
          frequency_offset,
          channel,
          noise,
          time,
          time_busy,
          time_rx,
          time_tx,
          time_bss_rx,
          time_scan
      })
  }
  fn from_nested_attribute_array(buffer: &[u8]) -> Vec<SurveyResult> {
    let mut results = vec![];
    let (_, attributes) = netlink::Attribute::unpack_all(buffer);
    if let Ok(rule) = SurveyResult::from_attributes(attributes) {
      results.push(rule);
    }
    results
}
}

pub struct SurveyInformation {
  interface: u32,
  results: Vec<SurveyResult>,
}

impl fmt::Display for SurveyInformation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for result in &(self.results) {
          writeln!(f, "{} {}", self.interface, result)?;
      }
      Ok(())
  }
}

impl SurveyInformation {
  pub fn from_message(message: &generic::Message) -> Result<SurveyInformation> {
      let mut interface = 0u32;
      let mut results = vec![];
      for attribute in &(message.attributes) {
          let id = Attribute::from(attribute.identifier);
          match id {
              Attribute::Ifindex => {
                interface = attribute.as_u32()?;
              }
              Attribute::SurveyInfo => {
                results = SurveyResult::from_nested_attribute_array(&attribute.as_bytes());
              }
              _ => (),
          }
      }
      Ok(SurveyInformation {
        interface,
        results,
      })
  }
}