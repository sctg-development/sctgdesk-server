#!/bin/bash
# Test if magick is installed
if command -v magick &> /dev/null
then
    echo "ImageMagick is installed."
else
    echo "ImageMagick is not installed. Please install it."
    exit 1
fi
magick appicon.svg -background none -transparent "#FFFFFF" -resize 1024x1024 -format png icon.png
magick icon.png -resize 81% -background none -gravity center -extent 1024x1024 icon_apple.png

for size in 16 32 48 64 128 256 512 1024; do
    #inkscape -z -o $size.png -w $size -h $size icon.svg >/dev/null 2>/dev/null
    magick icon.png -resize ${size}x${size} app_icon_$size.png
done

for size in 32 128 256 512 1024; do
    #inkscape -z -o $size.png -w $size -h $size icon.svg >/dev/null 2>/dev/null
    magick icon_apple.png -resize ${size}x${size} apple_icon_$size.png
done
# from ImageMagick
magick app_icon_16.png app_icon_32.png app_icon_48.png app_icon_128.png app_icon_256.png app_icon_512.png -colors 256 icon.ico
cp app_icon_32.png 32x32.png
cp app_icon_64.png 64x64.png
cp app_icon_128.png 128x128.png
cp app_icon_256.png 128x128@2x.png
#/bin/rm 16.png 32.png 48.png 128.png 256.png

# Test if png2icns is installed
if command -v png2icns &> /dev/null
then
    png2icns ../flutter/macos/Runner/AppIcon.icns apple_icon_1024.png apple_icon_512.png apple_icon_256.png apple_icon_128.png apple_icon_32.png
else
    echo "png2icns from libicns is not installed. Using simple monosize icns generation."
    magick apple_icon_256.png ../flutter/macos/Runner/AppIcon.icns
fi