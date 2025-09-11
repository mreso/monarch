use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the out directory
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Compile the .cu file
    let cu_file = Path::new("cuda_ping_pong.cu"); // Adjust path if needed
    let output_path = Path::new(&out_dir).join("cuda_ping_pong.o");

    let cu_file2 = Path::new("/mnt/home/mreso/monarch/rdmaxcel-sys/src/rdmaxcel.cu"); // Adjust path if needed
    let output_path2 = Path::new(&out_dir).join("rdmaxcel.o");
    let device_path = Path::new(&out_dir).join("device_ping_ping.o");

    let output_path3 = Path::new(&out_dir).join("libpingpong.a");

    
    // Run nvcc to compile the .cu file
    let status = Command::new("nvcc")
        .arg("-dc")                    // Compile only, don't link
        .arg("-I")
        .arg("../../../../")
        .arg("-o")                    // Output
        .arg(&output_path)            // Output file
        .arg(&cu_file)                // Input file
        .arg("-arch=sm_90")           // Adjust for your GPU architecture
        .arg("-std=c++20")
        .arg("--expt-extended-lambda")
        .arg("-Xptxas")
        .arg("-maxrregcount=96")
        .arg("-Xfatbin")
        .arg("-compress-all")
        .arg("--device-c")
        .arg("-Xcompiler")
        .arg("-fPIC,-Wno-deprecated-anon-enum-enum-conversion,-Wno-deprecated-enum-enum-conversion")
        .status()
        .expect("Failed to execute nvcc");
    
    if !status.success() {
        panic!("nvcc compilation failed");
    }

    // Run nvcc to compile the .cu file
    let status = Command::new("nvcc")
        .arg("-dc")                    // Compile only, don't link
        .arg("-I")
        .arg("../../../../")
        .arg("-o")                    // Output
        .arg(&output_path2)            // Output file
        .arg(&cu_file2)                // Input file
        .arg("-arch=sm_90")           // Adjust for your GPU architecture
        .arg("-std=c++20")
        .arg("--expt-extended-lambda")
        .arg("-Xptxas")
        .arg("-maxrregcount=96")
        .arg("-Xfatbin")
        .arg("-compress-all")
        .arg("--device-c")
        .arg("-Xcompiler")
        .arg("-fPIC,-Wno-deprecated-anon-enum-enum-conversion,-Wno-deprecated-enum-enum-conversion")
        .status()
        .expect("Failed to execute nvcc");
    
    if !status.success() {
        panic!("nvcc compilation failed");
    }

    // Run nvcc to compile the .cu file
    let status = Command::new("nvcc")
        .arg("-dlink")                // Input file
        .arg("-arch=sm_90")           // Adjust for your GPU architecture
        .arg("-o")                    // Output
        .arg(&device_path)            // Output file
        .arg(&output_path)                // Input file
        .arg(&output_path2)                // Input file
        .status()
        .expect("Failed to execute nvcc");
    
    if !status.success() {
        panic!("nvcc compilation failed");
    }
    let status = Command::new("ar")
        .arg("rcs")
        .arg(&output_path3)
        .arg(&output_path)
        .arg(&output_path2)
        .arg(&device_path)
        .status()
        .expect("Failed to execute nvcc");

    if !status.success() {
        panic!("static library creation failed");
    }
    
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=pingpong");
}