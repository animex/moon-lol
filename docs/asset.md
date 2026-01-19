# Asset Management Solutions

## Read Directly from WAD Files

### Advantages

1. No redundant resource files on disk
2. No extraction needed

### Disadvantages

1. Loading may be slow (unverified)
2. Cannot utilize bevy asset system
3. When WAD structure updates but game hasn't updated, cannot open old game version

## Extract Resources from WAD to Assets On-Demand

### Advantages

1. May load faster (unverified)
2. Utilize bevy asset system
3. WAD updates don't affect extracted files, can continue using
4. Unused resource files won't be extracted
5. Can convert to custom format, decoupling from League of Legends resource format and achieving higher loading performance

### Disadvantages

1. Need to extract files from WAD
2. Redundant resources on disk
3. On-demand extraction and conversion code is complex to implement

## Fully Extract WAD to Assets

### Advantages

1. May load faster (unverified)
2. Utilize bevy asset system
3. WAD updates don't affect extracted files, can continue using
4. Unused resource files won't be extracted
5. Extraction code is simple to implement

### Disadvantages

1. Need to extract files from WAD
2. Large amount of redundant resources on disk
3. Extraction takes too long
4. Unused resource files will also be extracted
5. Can only use League of Legends resource format completely
