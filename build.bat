@echo off

cd cpp_src
call build_C_lib
cd ..

cargo build --release