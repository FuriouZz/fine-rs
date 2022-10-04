use std::rc::Rc;

pub struct PipelineBuilder {
    device: Rc<wgpu::Device>,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    primitive: wgpu::PrimitiveState,
    multisample: wgpu::MultisampleState,
    depth_stencil: Option<wgpu::DepthStencilState>,
}

impl PipelineBuilder {
    pub fn new(device: Rc<wgpu::Device>) -> Self {
        Self {
            device,
            bind_group_layouts: Vec::new(),
            primitive: wgpu::PrimitiveState::default(),
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: None,
        }
    }

    pub fn create_bind_group_layout(
        mut self,
        entries: &[wgpu::BindGroupLayoutEntry],
    ) -> Self {
        self.bind_group_layouts
            .push(
                self.device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("PipelineBuilder::create_bind_group_layout"),
                        entries,
                    }),
            );
        self
    }

    pub fn set_primitive(&mut self, primitive: wgpu::PrimitiveState) {
        self.primitive = primitive;
    }

    pub fn set_multisample(&mut self, multisample: wgpu::MultisampleState) {
        self.multisample = multisample;
    }

    pub fn set_depth_stencil(&mut self, depth_stencil: Option<wgpu::DepthStencilState>) {
        self.depth_stencil = depth_stencil;
    }

    pub fn into_render_pipeline(
        self,
        vertex: wgpu::VertexState,
        fragment: Option<wgpu::FragmentState>,
    ) -> RenderPipeline {
        let pipeline_layout = self.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("PipelineBuilder"),
                bind_group_layouts: &self.bind_group_layouts.iter().collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });

        let pipeline = self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("PipelineBuilder"),
                layout: Some(&pipeline_layout),
                vertex,
                fragment,
                primitive: self.primitive,
                multisample: self.multisample,
                depth_stencil: self.depth_stencil,
                multiview: None,
            });

        RenderPipeline {
            device: self.device,
            pipeline,
            bind_group_layouts: self.bind_group_layouts,
        }
    }
}

pub struct RenderPipeline {
    device: Rc<wgpu::Device>,
    pipeline: wgpu::RenderPipeline,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
}

impl RenderPipeline {
    pub fn create_bind_group(
        &self,
        layout_index: usize,
        entries: &[wgpu::BindGroupEntry],
    ) -> Option<wgpu::BindGroup> {
        self.bind_group_layouts.get(layout_index).map(|layout| {
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("RenderPipeline::create_bind_group"),
                layout,
                entries,
            })
        })
    }

    pub fn use_pass<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
    }
}
