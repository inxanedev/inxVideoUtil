# inxVideoUtil

A small utility to quickly trim and compress a video using ffmpeg.  
It's made in Rust, using `egui` for the user interface.

# how to use
First, make sure to download `ffmpeg` and have it available in your system path.

1. Pause the video where you want to start or end, and press the appropriate button to set the position.
2. Press "Trim and Compress"

You can also customize the CRF value, to select how much you want the video to be compressed. A high CRF value, like 30 (which is the default), compresses the video a lot, and by extension - lower CRF values compress the video less.  
Setting CRF to 0 disables compression entirely and does not cause re-encoding.

# download
There is a precompiled binary in Releases, for x64 windows.

# compiling
First, install `ffmpeg`.
```
vcpkg install ffmpeg:x64-windows
```
Then, run `cargo build --release`  
Once it's built, copy the `.dll` files from `vcpkg/packages/ffmpeg_x64-windows/bin` to where your `inx_video_util.exe` was built, place them alongside the executable.  

Now it should be ready to run.

# adding to context menu
For easy access to the tool, you can add it to your Windows context menu.  
To do this, open the Registry Editor, and navigate to `HKEY_CLASSES_ROOT\*\shell`. In there, create a new Key, name it something like `Open in inxVideoUtil`. Inside it, create another key called `command`, and set the Default (REG_SZ) value in that key to the path where your .exe is, with "%1" added to the end.

Example:  
`C:\path\to\inx_video_util.exe "%1"`