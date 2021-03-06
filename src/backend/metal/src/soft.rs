use command::IndexBuffer;
use native::RasterizerState;

use hal;
use metal;

use std::ops::Range;


pub fn push_data(constants: &[u32]) -> Vec<u8> {
    constants
        .iter()
        .flat_map(|&v| (0 .. 4).map(move |i| (v >> 8*i) as u8))
        .collect()
}

#[derive(Clone, Debug)]
pub enum RenderCommand {
    SetViewport(metal::MTLViewport),
    SetScissor(metal::MTLScissorRect),
    SetBlendColor(hal::pso::ColorValue),
    SetDepthBias(hal::pso::DepthBias),
    BindBuffer {
        stage: hal::pso::Stage,
        index: usize,
        buffer: Option<metal::Buffer>,
        offset: hal::buffer::Offset,
    },
    BindBufferData {
        stage: hal::pso::Stage,
        index: usize,
        bytes: Vec<u8>,
    },
    BindTexture {
        stage: hal::pso::Stage,
        index: usize,
        texture: Option<metal::Texture>,
    },
    BindSampler {
        stage: hal::pso::Stage,
        index: usize,
        sampler: Option<metal::SamplerState>,
    },
    BindPipeline(
        metal::RenderPipelineState,
        Option<RasterizerState>,
        Option<metal::DepthStencilState>,
    ),
    Draw {
        primitive_type: metal::MTLPrimitiveType,
        vertices: Range<hal::VertexCount>,
        instances: Range<hal::InstanceCount>
    },
    DrawIndexed {
        primitive_type: metal::MTLPrimitiveType,
        index: IndexBuffer,
        indices: Range<hal::IndexCount>,
        base_vertex: hal::VertexOffset,
        instances: Range<hal::InstanceCount>,
    },
    DrawIndirect {
        primitive_type: metal::MTLPrimitiveType,
        buffer: metal::Buffer,
        offset: hal::buffer::Offset,
    },
    DrawIndexedIndirect {
        primitive_type: metal::MTLPrimitiveType,
        index: IndexBuffer,
        buffer: metal::Buffer,
        offset: hal::buffer::Offset,
    },
}

#[derive(Debug)]
pub enum BlitCommand {
    CopyBuffer {
        src: metal::Buffer,
        dst: metal::Buffer,
        region: hal::command::BufferCopy,
    },
    CopyImage {
        src: metal::Texture,
        dst: metal::Texture,
        region: hal::command::ImageCopy,
    },
    CopyBufferToImage {
        src: metal::Buffer,
        dst: metal::Texture,
        dst_desc: hal::format::FormatDesc,
        region: hal::command::BufferImageCopy,
    },
    CopyImageToBuffer {
        src: metal::Texture,
        src_desc: hal::format::FormatDesc,
        dst: metal::Buffer,
        region: hal::command::BufferImageCopy,
    },
}

#[derive(Debug)]
pub enum ComputeCommand {
    BindBuffer {
        index: usize,
        buffer: Option<metal::Buffer>,
        offset: hal::buffer::Offset,
    },
    BindBufferData {
        bytes: Vec<u8>,
        index: usize,
    },
    BindTexture {
        index: usize,
        texture: Option<metal::Texture>,
    },
    BindSampler {
        index: usize,
        sampler: Option<metal::SamplerState>,
    },
    BindPipeline(metal::ComputePipelineState),
    Dispatch {
        wg_size: metal::MTLSize,
        wg_count: metal::MTLSize,
    },
    DispatchIndirect {
        wg_size: metal::MTLSize,
        buffer: metal::Buffer,
        offset: hal::buffer::Offset,
    },
}

pub enum Pass {
    Render(metal::RenderPassDescriptor, Vec<RenderCommand>),
    Blit(Vec<BlitCommand>),
    Compute(Vec<ComputeCommand>),
}
