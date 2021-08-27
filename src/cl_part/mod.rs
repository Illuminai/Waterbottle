/*
use ocl::{Platform, DeviceType, Buffer, Queue, Context, Program, flags, Kernel};
use ocl::builders::{ContextBuilder};
use ocl::enums::DeviceSpecifier;
use ocl::Device;
use std::ffi::CString;
use ocl::core::KernelWorkGroupInfo;
use std::fmt::Debug;
use std::convert::TryInto;
use std::borrow::Borrow;


#[derive(Debug)]
pub struct CommentPair {
    pub(crate) begin: usize,
    pub(crate) end: usize,
    pub(crate) t: u8
}

pub(crate) fn run_opencl_part(text: &[u8]) -> ocl::Result<Vec<CommentPair>> {
    let (platform, device, context) = find_default_opencl_requirements()?;
    println!("Platform version: {}", platform.version().unwrap_or("Failed to get version".to_string()));

    let program = make_program(&context, &device, "").expect("Unable to create program");
    println!("Program status:");
    println!("\tLog: {:?}", program.build_info(device, ocl::core::ProgramBuildInfo::BuildLog)?);
    println!("\tBinType: {:?}", program.build_info(device, ocl::core::ProgramBuildInfo::BinaryType)?);
    println!("\tStatus: {:?}", program.build_info(device, ocl::core::ProgramBuildInfo::BuildStatus)?);
    println!("\tOptions: {:?}", program.build_info(device, ocl::core::ProgramBuildInfo::BuildOptions)?);

    let queue = Queue::new(&context, device, None).expect("Unable to create queue");

    let text_buffer = Buffer::<u8>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_READ_ONLY)
        .len(text.len())
        .copy_host_slice(&text)
        .build()?;

    const BUFFER_SIZE: i32 = 1024;

    let begin_buffer = Buffer::<i32>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_READ_WRITE)
        .len(BUFFER_SIZE)
        .build()?;
    let end_buffer = Buffer::<i32>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_READ_WRITE)
        .len(BUFFER_SIZE)
        .build()?;
    let type_buffer = Buffer::<u8>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_READ_WRITE)
        .len(BUFFER_SIZE)
        .build()?;
    let amount_buffer = Buffer::<i32>::builder()
        .queue(queue.clone())
        .flags(flags::MEM_READ_WRITE)
        .len(1)
        .build()?;

    let kernel = Kernel::builder()
        .program(&program)
        .name("find_pairs")
        .queue(queue.clone())
        .arg(&text_buffer)
        .arg(&begin_buffer)
        .arg(&end_buffer)
        .arg(&type_buffer)
        .arg(&amount_buffer)
        .global_work_size(1)
        .local_work_size(1)
        .build()?;

    let mut work_items= kernel.wg_info(device, KernelWorkGroupInfo::PreferredWorkGroupSizeMultiple)?;
    //work_items.into()
    println!("{:?}", work_items);

    let tmp;
    unsafe {

        tmp = kernel.cmd()
            .queue(&queue)
            .enq();
    }
    tmp?;

    queue.finish()?;

    let mut begins = vec![0i32; BUFFER_SIZE as usize];
    let mut ends = vec![0i32; BUFFER_SIZE as usize];
    let mut types = vec![0u8; BUFFER_SIZE as usize];
    let mut amount = vec![0; 1];
    begin_buffer.cmd().queue(&queue).offset(0).
        read(&mut begins).
        enq()?;
    end_buffer.cmd().queue(&queue).offset(0).
        read(&mut ends).
        enq()?;
    type_buffer.cmd().queue(&queue).offset(0).
        read(&mut types).
        enq()?;
    amount_buffer.cmd().queue(&queue).offset(0).
        read(&mut amount).
        enq()?;

    let mut ret = Vec::with_capacity(amount[0] as usize);

    for i in 0..(amount[0] as usize) {
        ret.push(CommentPair {
            begin: begins[i] as usize,
            end: ends[i] as usize,
            t: types[i]
        });
    }

    Ok(ret)
}

fn find_default_opencl_requirements() -> ocl::Result<(Platform, Device, Context)> {
    let platform = Platform::first()?;

    //Fetch first device
    let device = Device::list(platform, Option::Some(DeviceType::ALL)).expect("Unable to fetch devices").pop().expect("No devices available");
    let context = ContextBuilder::new().platform(platform).devices(DeviceSpecifier::Single(device)).build().expect("Unable to build context");

    Ok((platform, device, context))
}

fn make_program(context: &Context, device: &Device, compiler_options: &str) -> ocl::Result<Program> {
    //Leave empty for now
    let input_headers_map = std::collections::HashMap::<&str, &ocl::core::Program>::new();

    let mut input_header_names = Vec::<CString>::with_capacity(input_headers_map.len());
    let mut input_header_contents= Vec::<&ocl::core::Program>::with_capacity(input_headers_map.len());

    for (header_name, header_content) in input_headers_map {
        input_header_names.push(CString::new(header_name).expect("Unable to parse name"));
        input_header_contents.push(header_content);
    }

    let source_as_c_string = CString::new(include_str!("./resources/test.cl"))?;

    let program =ocl::core::create_program_with_source(context, &[source_as_c_string])?;
    let compiler_options_as_c_string = CString::new(compiler_options)?;

        //ocl::core::compile_program(&program, Option::Some(&[device]), &compiler_options_as_c_string, input_header_contents.as_slice(), input_header_names.as_slice(), None, None, None)?;
        //ocl::core::link_program(context, Some(&[device]), &compiler_options_as_c_string, &[&program], None, None, None)?;

    ocl::core::build_program(&program, Some(&[device]), &compiler_options_as_c_string, None, None)?;
    Ok(ocl::Program::from(program))
}
*/
