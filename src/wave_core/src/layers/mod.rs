/*
 MIT License

 Copyright (c) 2024 Nami Reghbati

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

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::utils::macros::logger::*;
#[cfg(feature = "debug")]
use crate::Engine;
use crate::EnumEngineError;
use crate::events::{self, EnumEvent, EnumEventMask};

pub mod window_layer;
pub mod renderer_layer;
pub mod imgui_layer;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum EnumLayerError {
  SyncDisabled,
  InvalidCallback,
  InvalidEventMask
}

impl Display for EnumLayerError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Layer] -->\t Error encountered in layer: Error => {0:?}", self)
  }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnumLayerType {
  Window = 0,
  Imgui = 1,
  Renderer = 2,
  App = 3,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum EnumSyncInterval {
  EveryFrame,
  Every(u32)
}

pub struct Layer {
  pub m_uuid: u64,
  pub m_name: &'static str,
  m_priority: u32,
  m_sync_polling_enabled: bool,
  m_sync_interval: EnumSyncInterval,
  m_poll_mask: EnumEventMask,
  pub(crate) m_data: Box<dyn TraitLayer>,
}

impl Eq for Layer {}

impl PartialEq<Self> for Layer {
  fn eq(&self, other: &Self) -> bool {
    return self.m_data.get_type() == other.m_data.get_type();
  }
}

impl PartialOrd<Self> for Layer {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Option::from(self.m_priority.cmp(&other.m_priority));
  }
}

impl Ord for Layer {
  fn cmp(&self, other: &Self) -> Ordering {
    return self.partial_cmp(other).unwrap();
  }
}

pub trait TraitLayer {
  fn get_type(&self) -> EnumLayerType;
  fn on_submit(&mut self) -> Result<(), EnumEngineError>;
  fn on_sync_event(&mut self) -> Result<(), EnumEngineError>;
  fn on_async_event(&mut self, event: &EnumEvent) -> Result<bool, EnumEngineError>;
  fn on_update(&mut self, time_step: f64) -> Result<(), EnumEngineError>;
  fn on_render(&mut self) -> Result<(), EnumEngineError>;
  fn on_free(&mut self) -> Result<(), EnumEngineError>;
  fn to_string(&self) -> String;
}

impl Layer {
  pub fn new<T: TraitLayer + 'static>(name: &'static str, data: T) -> Self {
    return Self {
      m_uuid: 0,
      m_name: name,
      m_priority: data.get_type() as u32,
      m_sync_polling_enabled: false,
      m_sync_interval: EnumSyncInterval::EveryFrame,
      m_poll_mask: EnumEventMask::None,
      m_data: Box::new(data),
    };
  }
  
  pub fn enable_sync_polling(&mut self) {
    self.m_sync_polling_enabled = true;
  }
  pub fn disable_sync_polling(&mut self) {
    self.m_sync_polling_enabled = false;
  }
  
  pub fn is_sync_enabled(&self) -> bool {
    return self.m_sync_polling_enabled;
  }
  
  pub fn get_type(&self) -> EnumLayerType {
    return self.m_data.get_type();
  }
  
  pub(crate) fn submit(&mut self) -> Result<(), EnumEngineError> {
    self.m_uuid = rand::random::<u64>();
    return self.m_data.on_submit();
  }
  
  pub fn is_type(&self, layer_type: EnumLayerType) -> bool {
    return self.m_data.get_type() == layer_type;
  }
  
  pub fn is_named(&self, name: &str) -> bool {
    return self.m_name == name;
  }
  
  pub(crate) fn get_poll_mask(&self) -> EnumEventMask {
    return self.m_poll_mask;
  }
  
  pub fn enable_async_polling_for(&mut self, event_mask: EnumEventMask) {
    self.m_poll_mask = event_mask;
  }
  
  /// Set a specific interval for polling synchronous events. This is useful when
  /// you don't want to poll events on every frame due to the overhead, but still require the
  /// sync api for a synchronous task (i.e. Physics, Pathfinding, etc...).
  ///
  /// ### Args:
  /// - *interval*: A positive number indicating the number of frames to wait before polling sync events.
  /// In other words, a value of [EnumSyncInterval::EveryFrame] suggests polling on every frame, while
  /// [EnumSyncInterval::Every(1)] would indicate ever other frame.
  ///
  /// ### Returns:
  /// - *Result<(), [EnumLayerError]>*: The unit value if successful, otherwise [EnumLayerError::SyncDisabled].
  pub fn set_sync_interval(&mut self, interval: EnumSyncInterval) -> Result<(), EnumLayerError> {
    if !self.is_sync_enabled() {
      log!(EnumLogColor::Red, "ERROR", "[Layer] -->\t Cannot set sync interval, sync polling is not enabled!");
      return Err(EnumLayerError::SyncDisabled);
    }
    self.m_sync_interval = interval;
    return Ok(());
  }
  
  pub fn get_sync_interval(&self) -> u32 {
    return match self.m_sync_interval {
      EnumSyncInterval::EveryFrame => 0,
      EnumSyncInterval::Every(count) => count
    };
  }
  
  pub(crate) fn poll_includes(&self, poll_mask: EnumEventMask) -> bool {
    return self.m_poll_mask.contains(poll_mask);
  }
  
  pub(crate) fn polls(&self, event: &EnumEvent) -> bool {
    let cast = events::EnumEventMask::from(event);
    return self.m_poll_mask.contains(cast);
  }
  
  pub fn try_cast<T: TraitLayer + 'static>(&self) -> Option<&T> {
    return unsafe { Some(&*(self.m_data.deref() as *const dyn TraitLayer as *const T)) };
  }
  
  pub(crate) fn on_sync_event(&mut self) -> Result<(), EnumEngineError> {
    return self.m_data.on_sync_event();
  }
  
  pub(crate) fn on_async_event(&mut self, event: &EnumEvent) -> Result<bool, EnumEngineError> {
    return self.m_data.on_async_event(event);
  }
  
  pub(crate) fn on_update(&mut self, time_step: f64) -> Result<(), EnumEngineError> {
    return self.m_data.on_update(time_step);
  }
  
  pub(crate) fn on_render(&mut self) -> Result<(), EnumEngineError> {
    return self.m_data.on_render();
  }
  
  pub(crate) fn free(&mut self) -> Result<(), EnumEngineError> {
    return self.m_data.on_free();
  }
  
  pub fn to_string(&self) -> String {
    return self.m_data.to_string();
  }
}

impl Display for Layer {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "'{1}'\n{0:113}UUID: {2}\n{0:113}Poll mask: {3}\n{0:113}Sync polled?: {4}\
    \n{0:113}Priority: {5}\n{0:113}Data: {6}", "",
      self.m_name, self.m_uuid, self.m_poll_mask, self.m_sync_polling_enabled, self.m_priority, self.m_data.to_string())
  }
}