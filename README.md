# wdct
Wallpaper Downloader CLI Tool

### Build
```cargo build --release```

### Usage
```wdct --search-query <Tags> [Sorting]```

You can use multiple tags if you quote them. Sorting is optional and it will be based on date_added by default. 

Despite what the name implies, this tool doesnt really download the wallpaper. it prints the paths for you to download(using tools like ```wget```).
**Example**: ```wget $(wdcs fantasy views)```
