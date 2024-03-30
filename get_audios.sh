#!/bin/bash

# Define the URL of the zip file and the folder name
ZIP_URL="https://everyayah.com/data/translations/urdu_shamshad_ali_khan_46kbps/000_versebyverse.zip"
FOLDER_NAME="urdu"
# ZIP_URL="https://everyayah.com/data/English/Sahih_Intnl_Ibrahim_Walk_192kbps/000_versebyverse.zip"
# FOLDER_NAME="ENGLISH"
# ZIP_URL="https://everyayah.com/data/Alafasy_128kbps/000_versebyverse.zip"
# FOLDER_NAME="URDU"


#extract data is respective folders
PARENT_DIR="resources/audios/"
DEST_DIR="resources/audios/$FOLDER_NAME"
wget "$ZIP_URL" -P "$PARENT_DIR"
mkdir -p "$DEST_DIR"
unzip -o "$PARENT_DIR/000_versebyverse.zip" -d "$DEST_DIR"
rm "$PARENT_DIR/000_versebyverse.zip"
echo "Download and extraction completed successfully."

