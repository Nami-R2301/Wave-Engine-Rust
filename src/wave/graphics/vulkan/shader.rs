/*
///////////////////////////////////   Vulkan shader    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
///////////////////////////////////                    ///////////////////////////////////
 */

use crate::wave::graphics::shader::{TraitShader, EnumError};

#[derive(Debug, Clone, PartialEq)]
pub struct VkShader {
  pub m_id: u32,
  pub m_vertex_str: String,
  // For debug purposes.
  pub m_fragment_str: String,
  m_uniform_cache: std::collections::HashMap<&'static str, i32>,
}

impl TraitShader for VkShader {
  fn new(_vertex_file_path: &'static str, _fragment_file_path: &'static str) -> Result<Self, EnumError> where Self: Sized {
    return Ok(VkShader {
      m_id: 0,
      m_vertex_str: "Empty".to_string(),
      m_fragment_str: "Empty".to_string(),
      m_uniform_cache: std::collections::HashMap::new()
    });
  }
  
  fn from(_other_shader: Self) -> Self where Self: Sized {
    todo!()
  }
  
  fn compile(&self, _shader_id: u32, _shader_type: &dyn std::any::Any) -> Result<(), EnumError> {
    todo!()
  }
  
  fn send(&mut self) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn upload_data(&mut self, _uniform_name: &'static str, _uniform: &dyn std::any::Any) -> Result<(), EnumError> {
    return Ok(());
  }
  
  fn get_id(&self) -> u32 {
    return self.m_id;
  }
  
  fn to_string(&self) -> String {
    return format!("Vertex shader => {0}\n{1:113}Fragment shader : \n{2}", self.m_vertex_str,
      "", self.m_fragment_str);
  }
}