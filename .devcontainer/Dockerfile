FROM mcr.microsoft.com/devcontainers/rust:latest

# Include lld linker to improve build times either by using environment variable or by setting
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" of with Cargo's confirguration file. (i.e. .cargo/config.toml)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean