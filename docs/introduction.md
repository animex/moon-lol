# Bevy

0.17.0

# Game Configuration

Extract game resources from League of Legends WAD files

DATA/FINAL/Maps/Shipping/Map11.wad.client

This is the main map WAD file currently being read, storing the file table and file contents.

The file table records:

1. Hash value of each file's path
2. File's offset in the .wad.client binary data
3. File's size
4. Other information

Uses u64 hash to store paths. For example, if a file's path is data/maps/mapgeometry/map11/bloom.mapgeo, this file contains all models in the current version's map.

You can calculate this path's hash value as 0xe8b4704f422901d9 using the hash algorithm. This hash algorithm was obtained from the open-source LeagueToolkit on GitHub.

Next, you can use 0xe8b4704f422901d9 to find the file's offset and size,

Then you can read the file's contents from the .wad.client binary data.

These files mainly consist of model files, texture files, animation files, and configuration files. Configuration files basically use .bin as the suffix.

Barrack

# Left-handed -> Right-handed Coordinate System

League of Legends uses a left-handed coordinate system, while Bevy uses a right-handed coordinate system, so the camera's screen clip space needs to be horizontally flipped.
