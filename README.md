# mpd_queue_stats

This program, written in Rust, returns a string for use by Conky. The string is in the following format:
```
"{}${{alignc}}{}${{alignr}}${{offset -5}}{}\n$alignc${{voffset -40}}${{offset 30}}{:.2}%"
```
...where each `{}` is replaced by the following values in this order:

    1. Elapsed time
    2. Total length of the queue
    3. Remaining time
    4. Percent elapsed

Note that all values relate to the queue, not the currently playing song, although the 'Elapsed time' also includes the elapsed time of the current song.

In conky, this is invoked this way:
```
${execpi n (mpd_queue_stats)}
```
...where n is an integer defining how often conky calls this function. Note that either mpd_queue_stats must be on the $PATH available to Conky when it is run, or you must include a complete path to the mpd_queue_stats executable. e.g.
```
${execpi n (/home/username/mybin/mpd_queue_stats)}
```

Note there is a line-break, and then the string sends the cursor to the middle of the line and then adds an offset of 50 pixels before outputting the percent. This is what I needed for my conky setup. If your needs differ, adjust the output string to suit you. My .conkymusic file sets the window to 280 pixels wide.

## Building
If you are unfamiliar with Rust, the folder `mpd_queue_stats` is the root of the crate. A crate describes a project and its dependencies. The actual code is located in the `mpd_queue_stats/src` folder, and is named `mpd_queue_stats.rs`. In this file is a constant which needs changing if your MPD is not running on the same machine as your instance of Conky, or uses a different port. The line looks like this:
```
const HOST: &str = "localhost:6600";
```

Assuming you have installed Rust, then from the root of the mpd_queue_stats folder:
```
$ cargo ret
```
If everything goes well:
```
$ cargo build --release
```
The executable will be found as `mpd_queue_stats/target/release/mpd_queue_stats`.

## See Also
1. [mpd_queue_len](https://github.com/stroggprog/mpd_queue_len)
2. [mpd_queue_stats](https://github.com/stroggprog/mpd_queue_stats)
3. [mpd_percent](https://github.com/stroggprog/mpd_percent)
