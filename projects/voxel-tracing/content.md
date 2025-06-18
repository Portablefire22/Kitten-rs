It has been too long since I've last touched anything gaming related, been too long since 
I've touched voxels, and it has definitely been too long since I've gained suicial thoughts 
from figuring out raytracing. You might look at that and wonder why I'm even trying this 
but it's actually quite fun once you get past all of the pain.

This website looks a bit bare - even with the new posts - so I thought I'd add a 
web compatible project on here, of course that means working with [wgpu](wgpu.rs) 
and losing things like multi-threading but it's maybe worth it for something that 
looks cool on a boring website.

# Initialising the Window and Rendering 

This ain't a tutorial, and I don't have the knowledge to provide a guide on how to setup 
wgpu, so I'll point you to the resources I used to get everything setup. 
[Ben Hansen's learn wgpu](https://sotrh.github.io/learn-wgpu/) page is what I used 
for all of the boilerplate and seems to be great for any wgpu projects, I'm actually 
fairly certain I've used it before without realising.

## Problems 

These aren't problems with the guide, they're just problems I encountered and I thought 
it best to talk about them in case I want to revist this again.

### WebGPU support 

As of now, 2025-06-17, WebGPU is supported on: Chromium, though arguments are needed 
if launched on Linux; Firefox, only on nightly; and apparently Safari, though it is 
quite experimental. Anything else probably won't work, so unfortunately you'll have 
to use Chromium.

From my experience running chromium via the following command fixes GPU support.

```chromium --enable-unsafe-webgpu --enable-features=Vulkan```

### CORS Policy

ES6 modules can't be accessed through the "file://" protocol, which forces us to 
use a webserver to host the content for the browser to pull from. This isn't an 
issue in production since we have to use a web server - can't exactly have our 
users downloading "kitten.rs/projects/voxel%20tracing.tar.gz" over FTP - but 
it can be slightly annoying when testing on a plain local index.html page.

# Displaying to the screen

Okay so we're not actually going to do any triangle rasterisation in this project 
other than two triangles to create a single plane. All we need from the tutorial is 
creating a two triangle plane with a texture, both of which are covered by the tutorial 
if you stop after "Textures and Bind Groups". We're going to skip the rasterisation step 
for the rendering by using a Compute shader - getting our GPU to do ***a lot*** of 
calculations - and saving the data from that to a texture, which gets displayed on 
our singular plane. 

Add `wgpu::TextureUsages::Storage_Binding` to the view texture's usage, create a 
compute pipeline with the following bind group layout. 

```rust
let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("Compute Bind Group Layout"),
    entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::StorageTexture { 
            access: wgpu::StorageTextureAccess::WriteOnly, 
            format: wgpu::TextureFormat::Rgba8Unorm, 
            view_dimension: wgpu::TextureViewDimension::D2, 
        },
        count: None,
    }],
});
```

Then set the bind group entry to the texture view and you've got a compute shader 
pipeline that can write to a texture that is being actively displayed to the screen. 
Now I've moved all of this into two functions - one for creating the texture, and another 
for creating the compute pipeline - since the texture needs to scale with resolution and 
we can't just modify the parameters during operation. Like rendering the compute pipeline 
will need to be submitted to the queue and the data returned should be polled since it's 
our display - don't worry about any issues with blocking, this is the same as waiting for 
a frame to render with traditional rasterisation - and then rendering should occur after 
the compute. I've probably done a terrible job at explaining this, so just have the code 
I added to make this happen.

```rust
fn compute(&mut self) -> Result<(), wgpu::Error> {
    let mut compute_encoder = self
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });
    {
        let mut compute_pass = compute_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        let size = self.window.inner_size();
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
        compute_pass.insert_debug_marker("Compute iterations");
        compute_pass.dispatch_workgroups(size.width, size.height, 1);
    }
    self.queue.submit(Some(compute_encoder.finish()));

    self.device.poll(wgpu::MaintainBase::wait()).unwrap();

    Ok(())
}

fn new_view_compute(device: &wgpu::Device, view_texture_view: wgpu::TextureView) -> (wgpu::ComputePipeline, wgpu::BindGroupLayout, wgpu::BindGroup) {
    let compute_shader = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

    let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture { 
                access: wgpu::StorageTextureAccess::WriteOnly, 
                format: wgpu::TextureFormat::Rgba8Unorm, 
                view_dimension: wgpu::TextureViewDimension::D2, 
            },
            count: None,
        }],
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &compute_shader,
        entry_point: Some("main"),
        cache: None,
        compilation_options: wgpu::PipelineCompilationOptions::default()
    });

    let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &compute_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view_texture_view),
            }
        ]
    });
    (compute_pipeline, compute_bind_group_layout, compute_bind_group)
}

fn new_view_texture(device: &wgpu::Device, mut width: u32, mut height: u32) -> (wgpu::BindGroup, wgpu::BindGroupLayout, wgpu::TextureView) {
    if width == 0 || height == 0 {
        width = 10;
        height = 10;
    }
    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let view_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::STORAGE_BINDING,
        label: Some("View Texture"),
        view_formats: &[],
    });

    let view_texture_view = view_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let view_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let view_texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("View Texture Bind Group Layout"),
        entries: &[
           wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false, 
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    (device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("View Texture Bind Group"),
        layout: &view_texture_bind_group_layout, 
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&view_sampler),
            }
        ],
    }), view_texture_bind_group_layout, view_texture_view)
}
```

```rust
// fn window_event
// ..
WindowEvent::RedrawRequested => {
    state.compute();
    match state.render() {
// ..
```

Slap that into the code, code a compute shader that marches a ray to draw a circle, 
and you've got a circle on the screen!

![broken sphere](/assets/voxel-oops.png)

*Thats not a circle...*

Okay turns out I just had the texture coordinates a bit wrong, correcting those - 
plus adding a temp ground -, brings us this!

![working sphere](/assets/voxel-sphere.png)

An actual sphere rendering based entirely on it's distance away from 0,0,0 and it's 
only slightly clipping the ground plane at y:0.25 :D. Now we can basically render 
anything that can be defined with a "signed distance field" - henceforth called an 
"SDF" - as long as we add it to the compute shader. Want a box? Have a box! 

![working cube](/assets/voxel-cube.png)

The best part of this is that I don't need to do any maths myself, I can just convert 
[Inigo Quilez's distance functions](https://iquilezles.org/articles/distfunctions/) 
to WGSL and I have that shape now rendering on my screen. In testing I added the 
"Box frame - exact" function to my compute shader and the output can be found below.

![working frame](/assets/voxel-frame.png)

Now all I have to do is "just" implement some form of camera movement/control and 
a way of sending voxel data to the GPU for it to render with a cube SDF. A few sections 
for you no doubt, hours or maybe days for me :)
