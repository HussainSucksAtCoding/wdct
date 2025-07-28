# wdcs
Wallpaper Downloader CLI Script

### Build
```cargo build --release```

### Usage
```wdcs <Wallpaper tags> [Sorting]```

You can use multiple tags. Sorting is optional and it will be based on date_added by default. 

Despite what the name of the repo implies, this script doesnt really download the wallpaper. it prints the paths for you to download(using tools like ```wget```).
**Example**: ```wget $(wdcs fantasy views)```
