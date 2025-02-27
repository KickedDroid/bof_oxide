#!/usr/bin/env python3
import os
import sys
import subprocess

def run_command(cmd, print_cmd=True):
    """Run a shell command and handle errors"""
    if print_cmd:
        print(f"Running: {' '.join(cmd)}")
    try:
        result = subprocess.run(cmd, check=True, text=True, capture_output=True)
        if result.stdout.strip():
            print(result.stdout)
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error: Command failed with exit code {e.returncode}")
        print(f"Output: {e.stdout}")
        print(f"Error: {e.stderr}")
        return False

def build(features):
    """Build the program with the given features"""
    # Create objects directory if it doesn't exist
    if not os.path.exists("objects"):
        os.makedirs("objects")
    
    # Prepare features
    rust_features = []
    c_features = []
    
    # Define mapping of features to C feature flags
    feature_to_c_flag = {
        "data": "-DDATA",
        "process_injection": "-DPROCESS_INJECTION",
        "format": "-DFORMAT"
        # Add more mappings as needed
    }
    
    # Add features based on args
    for feature in features:
        # Add Rust feature
        rust_features.append(f"--cfg")
        rust_features.append(f'feature="{feature}"')
        
        # Add corresponding C feature if it exists in our mapping
        feature_lower = feature.lower()
        if feature_lower in feature_to_c_flag:
            c_features.append(feature_to_c_flag[feature_lower])
    
    print(f"C Features: {' '.join(c_features)}")
    
    # Compile Rust part
    rustc_cmd = [
        "rustc",
        "--target", "x86_64-pc-windows-gnu",
        "-C", "opt-level=z",
        "-C", "panic=abort",
        "-C", "debuginfo=0",
        "-C", "strip=symbols",
        "-C", "codegen-units=1",
        "-C", "embed-bitcode=no",
        "-C", "target-cpu=x86-64",
        "-C", "target-feature=+crt-static",
        "-C", "link-arg=-nostartfiles",
        "-C", "link-arg=-nodefaultlibs",
        "-C", "link-arg=-Wl,--gc-sections",
        "--emit=obj",
        "src/lib.rs",
        "-o", "objects/rust_part.o"
    ] + rust_features
    
    if not run_command(rustc_cmd):
        return False
    
    # Compile C part
    gcc_cmd = [
        "x86_64-w64-mingw32-gcc",
        "-c",
        "-DOUTPUT",
    ] + c_features + [
        "src/entry.c",
        "-o", "objects/c_part.o"
    ]
    
    if not run_command(gcc_cmd):
        return False
    
    # Link objects
    ld_cmd = [
        "x86_64-w64-mingw32-ld",
        "-r",
        "objects/rust_part.o",
        "objects/c_part.o",
        "-o", "objects/combined.o"
    ]
    
    if not run_command(ld_cmd):
        return False
    
    # Final objcopy
    objcopy_cmd = [
        "x86_64-w64-mingw32-objcopy",
        "--remove-section=.drectve",
        "--strip-symbol=@feat.00",
        "--remove-section=.data",
        "--remove-section=.bss",
        "--strip-symbol=rust_begin_unwind",
        "--strip-debug",
        "objects/combined.o",
        "bof_oxide.o"
    ]
    
    return run_command(objcopy_cmd)

if __name__ == "__main__":
    # Check arguments
    if len(sys.argv) < 2:
        print("Usage: gen.py [feature1] [feature2] ...")
        print("Available features:\n- format\n- data\n- process_injection")
        sys.exit(1)
    
    # Get features from command line (skipping script name)
    raw_features = sys.argv[1:]
    
    # Process features, handling comma-separated values
    features = []
    for feature in raw_features:
        # Split by comma and strip whitespace
        split_features = [f.strip() for f in feature.split(',')]
        # Add non-empty features to the list
        features.extend([f for f in split_features if f])
    
    # Run the build
    success = build(features)
    
    # Exit with appropriate code
    sys.exit(0 if success else 1)
