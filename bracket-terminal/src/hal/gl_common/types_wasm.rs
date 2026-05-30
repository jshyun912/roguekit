pub type TextureId = glow::WebTextureKey;
pub type BufferId = glow::WebBufferKey;
pub type VertexArrayId = glow::WebVertexArrayKey;
pub type ShaderId = glow::WebProgramKey;
#[cfg(not(target_arch = "wasm32"))]
pub type FramebufferId = glow::WebFramebufferKey;
