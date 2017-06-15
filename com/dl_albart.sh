#!/bin/sh

url_AlbumArt=$("youtube-dl --playlist-items 1 --get-thumbnail $url")
$(wget $url_AlbumArt)

#youtube-dl --list-thumbnails https://www.youtube.com/playlist?list=PLPiidWAbJJA7YYFABxoACWZOgUbQpHuA2
# wget ^
