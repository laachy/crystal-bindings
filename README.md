# Crystal palace library binding generator 
Crystal Bindings is designed for generating bindings for anything in a modular fashion. This project is specifically designed for [crystal palace linker](https://tradecraftgarden.org/crystalpalace.html) for cross language bindings but with a few modifications can be used for anything.

This crate provides an easy way to develop [PICOs](https://tradecraftgarden.org/docs.html#picos) in rust using existing c libraries to generate the external interface that is called through rust. This binder generates separate modules per header file, these modules are located within the target folder (OUT_DIR). It is designed to be used dynamically at run time without any modifications.

Bindings such as these are called *-sys bindings. A better, safer alternative would be to take these bindings and use safer rust types. Again, this would require manual work and I like automated solutions allowing a "just works" solution.

Another approach to using this crate is to generate bindings once and save them to use without generation ever again. While bindings are only generated once unless the underlying header is modified, maybe you want to modify them or reference them in a certain way. Therefore this is another use case and valid approach.
 

Examples of usage can be found [here](https://github.com/laachy/tradecraft-garden-rs)


# Usage
### Adding the crate to your project

   ```powershell
   cargo add crystal-bindings
   ```
   
### Binding libraries
Binding libraries requires an environment variable called `HEADERS_PATH`to be set for the absolute path to the "to be binded" headers.

I found the easiest way to be to set an env inside .cargo/config.toml as such:

    [env]
    HEADERS_PATH = { value = "folder", relative = true }

# Other notes

Types and definitions such as from windows are injected at build time.

### Injecting c definitions

Inside `sdk/types.h` any types can be injected from libraries or manually. These are used by clang to deduce types. I have put windows.h in as an example and use case since many types and functions are used from it

### Injecting rust types

Inside `src/types.rs` lies the exposed rust type interface. Any types exposed inside here can be used by the generated bindings. I have only exposed types I have currently needed, this may need to be modified manually. 
