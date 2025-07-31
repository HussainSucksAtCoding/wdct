# wdct
Wallpaper Downloader CLI Tool

### Build
```cargo build --release```

The executable should spawn in target/release

### Usage
```wdct --tags <Tags> --sorting [Sorting]```

You can use multiple tags if you quote them. Sorting is optional and it will be based on date-added by default.

**Example:** ```wdct --tags "anime fantasy" --sorting random```

Despite what the name implies, this tool doesnt really download the wallpaper. it prints the paths for you to download(using tools like ```wget```).
**Example**: ```wget $(wdcs --tags fantasy --sorting views)```
