# Constellation

Constellation is a new hobby operating system written in Rust for the `x86_64` and `aarch64` platforms. It is currently in a very early stage of development and consists of only a few libraries and the Boötes bootloader, which is also still under development.

Over time, as the project continues to grow, additional components of the operating system will be added — such as the kernel, modules and drivers, and eventually the user space.

## Kernel

The Constellation kernel does not yet exist (nor does it have a name), but it is expected to have a modular architecture based on a microkernel that will operate on the principle of least privilege. This means that each kernel module and/or driver will have access only to what it needs. In addition, some modules and drivers will be loaded into user space to ensure better systen protection and security.

The kernel itself will contain only systems necessary for the operation and startup of the system.
