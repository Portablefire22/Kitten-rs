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

I was getting bored of the simple flat lighting and wanted something with a tiny bit 
more deatil, so I read through 
[Maxime Heckel's "Painting With Math"](https://blog.maximeheckel.com/posts/painting-with-math-a-gentle-study-of-raymarching/) 
blog post to add a simple form of diffuse lighting to my scene in the hope that it would 
provide more visual information. It's a bit weird on boxes but seems to work perfectly 
on spheres and provides just enough visual information to be useful for my usecase.

[![diffuse lighting video](http://img.youtube.com/vi/4Y8m1Xma1NY/0.jpg)](http://www.youtube.com/watch?v=4Y8m1Xma1NY "2025 06 24 21 06 47")
*The above image should link to a video*

# Creating the Scene 

A cube and sphere aren't all that interesting so it's probably best that I implement a 
voxel scene to render to. 

## Octree

Marching our rays through a scene would get massively expensive as more objects get 
added to the screen, we'd basically have to query every object in VRAM for their 
distance to the ray every time we march. The solution to this is to use a tree 
data structure that allows us to group voxels by their position, and only query 
voxels that are within nodes that the ray is currently intersecting. I am certain there 
are plenty of data structures that would accomplish this but the general approach seems 
to be using an "Octree" to create a "Sparse Voxel Octree" for containing voxel data 
for raymarching/tracing. Nvidia basically wrote the book on Sparse Voxel Octrees - 
henceforth referred to as SVOs - in their paper ["Efficient Sparse Voxel Octrees - 
Anaylsis, Extensions, and Implementation](https://research.nvidia.com/sites/default/files/pubs/2010-02_Efficient-Sparse-Voxel/laine2010tr1_paper.pdf)
where they discuss the differences between triangle-based geometry and voxel-based 
geometries and how SVOs function. 

Okay now we've done the "informational" section, let's switch to the part where I 
actively lose my mind from being heavily skill-issued. As of writing - 2025-06-24 - 
I am completely confused as to how to write an octree that fuffils the criteria of 
being written in Rust, and existing in a manner that can be represented on the GPU. 
It doesn't seem like it'd be that difficult - especially since it's just copying 
data from the CPU side to the GPU and that's something we've already done with 
the view texture - but WGSL just doesn't support recursive data types and I 
don't know how to implement an Octree in a non-recursive manner. I see people 
mention using 3D textures but they don't elaborate much further than that, idk 
I guess the next section will be when I finally understand and can explain it?

### The Approach

I'm going into this nearly blind, like I have read a few Nvidia articles and 
maybe a few blog posts to get this knowledge. Working off section 
"37.1.2 Implementation" from 
[Nvidia's GPU Gems 2](https://developer.nvidia.com/gpugems/gpugems2/part-v-image-oriented-computing/chapter-37-octree-textures-gpu)
has given me the rough idea of how to create an Octree implementation that can 
exist on the GPU. A quick synposis of the approach is basically to create a buffer 
that contains all Octree data - this data contains all information for nodes and 
data from leafs - and should be completely accessible by using array indices in 
place of traditional pointers on children nodes. Probably a garbage explination so 
I'll come back through at the end to explain it better, if you're seeing this then 
I forgot to do that or I concluded that this explanation was good enough.

Welp, time to implement this shit in Rust :)

### Creating A Tree In Rust 

Tree structures are questionable in Rust due to the borrow checker and the complete 
lack of traditional pointers making recursive types a pain to implement, don't 
believe me? Look at how you impement a linked list in rust - it's not a tree 
structure but it gets the point across that traditional data structures are a 
pain to implement. With this in mind, I think I'll try to implement the Rust side 
of things like how the GPU will store the data so that CPU-side conversions 
will not be required. This part is actually kind of fun because both you and I 
don't actually know the outcome of this approach without skipping ahead.

The implementation in the book is intended for applying a texture to an object via 
an octree and therefore only makes use of the RGBA channels of an image texture. This 
isn't what I want but the underlying theory ***should*** be enough for me to create a 
version that holds actual Voxel data that I require - not sure what the Voxel data 
is right now but I don't see why it couldn't be expandable.

I've decided to use 32-bit unsigned integers for holding the data in the array for a 
simple reason, the first 24 bits can be used to hold the RGB values of each voxel - 
I don't intend to use textures on voxels in this engine since I want the voxels 
themselves to texturise the environment - and the final 8 bits can be used as a 
bit field to handle the appearance of each voxel. The final 8 bits might not sound like 
much but they could be configured to hold 8 individual toggles or expanded so that an 
exclusive toggle has access to 7 bits of data. This system is not final but it *seems* 
like it should provide enough for simple rendering :) Furthermore, the nice part of using 
this system is that u32's should theoretically allow for an octree with 4,294,967,295 
leafs, which should be more than enough for what we need since adapting a previous 
project to use this system would allow for 131,071 32x32x32 chunks to be in a single 
octree.

Some quick face-rolling on the keyboard and I come up with the following data types:

```rust

#[derive(Clone, Copy, Debug)]
pub(crate) enum NodeType {
    Leaf(u32),
    Node(u32),
}

pub(crate) struct LeafData {
    colour: [u8; 3],
    transparent: bool,
}

impl Into<u32> for LeafData {
    fn into(self) -> u32 {
        ((self.colour[0] as u32) << 16) 
            & ((self.colour[1] as u32) << 8) 
            & (self.colour[2] as u32) 
            & ((self.transparent as u32) << 7)
    }
}

impl Into<u32> for NodeType {
    fn into(self) -> u32 {
        match self {
            NodeType::Leaf(data) => data.into(),
            NodeType::Node(index) => index,
        }
    }
}

pub(crate) struct Octree {
    data: Vec<NodeType>,
    position: Vec<f32>,
}

impl Into<Vec<u32>> for Octree {
    fn into(self) -> Vec<u32> {
        self.data.iter().map(|x| (*x).into()).collect()   
    }
}
```

Writing this section has made me realise that there is actually no way to determine 
if an entry is an index or leaf so let's fix that by just making the right most bit 
determine wether to use the value as an index or not. Simple change, it just means 
we now only have 31 bits for the array index and only 7 bits for the data bit field.
Since nodes are going to be the most common I opted to make a value of '1' indicate 
that the value points to another node on the octree. I hate future me so I've opted 
to just cutoff the final bit of Leaf nodes and deal with the consequences later.

```rust
impl Into<u32> for NodeType {
    fn into(self) -> u32 {
        match self {
            NodeType::Leaf(data) => (u32::from(data) & 0xFE) ,
            NodeType::Node(index) => (index | 1),
        }
    }
}
```
