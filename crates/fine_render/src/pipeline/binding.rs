pub struct BindGroupLayoutBuilder(pub(crate) Vec<wgpu::BindGroupLayoutEntry>);

impl BindGroupLayoutBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// A description of a single binding inside a bind group.
    pub fn set_entry(&mut self, entry: wgpu::BindGroupLayoutEntry) {
        self.0.push(entry);
    }
}

pub struct BindGroupBuilder {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
    layout: wgpu::BindGroupLayout,
}

impl BindGroupBuilder {
    /// Return bind group layout
    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    /// Bind resources to entries
    pub fn bind<'a, F>(&'a self, f: F) -> Vec<wgpu::BindGroupEntry>
    where
        F: Fn(&wgpu::BindGroupLayoutEntry) -> Option<wgpu::BindingResource<'a>>,
    {
        self.entries
            .iter()
            .filter_map(|entry| match f(entry) {
                Some(resource) => Some(wgpu::BindGroupEntry {
                    binding: entry.binding,
                    resource,
                }),
                None => None,
            })
            .collect()
    }
}