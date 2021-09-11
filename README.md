How does docker work?  That's what I am trying to figure out, this is a barebone 
implementation of a container system.

The goal is to run an appication in an environment where it can not interact with
other processes or data.  At least provided that there aren't any issues in the
operating system.

There should really only be a single standalone executable that is able to access
the stdin and stdout of the parent jail process.

### Development Environment

-   The system itself needs to be statically linked, since no dynamic linker will be
    avaliable.  Currently, this only seems to be possible with musl libc.  Let's install the
    toolchain:

    ~~~none
    rustup target add x86_64-unknown-linux-musl
    ~~~

-   Create build directory:

    ~~~none
    meson setup build --cross-file cross.ini
    cd build/
    ~~~

### Build Instructions

~~~none
meson compile
sudo ./jail
~~~
