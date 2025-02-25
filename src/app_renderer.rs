use egui_wgpu::wgpu;

pub struct AppRenderer {
    clear_color: wgpu::Color,
    // render_pipeline: wgpu::RenderPipeline,
}

impl AppRenderer {
    pub fn new(clear_color: wgpu::Color) -> Self {
        Self { clear_color }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface_view: &wgpu::TextureView,) {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Background Clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            
    }
}