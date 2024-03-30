## Personal utility for creating quranic shorts

### Install ffmpeg (Ubuntu)
sudo apt update && sudo apt upgrade
sudo snap install ffmpeg
ffmpeg -version

### configuration in case of manual installation
./configure --prefix= --prefix=/usr --disable-debug --disable-doc --disable-static --enable-cuda --enable-cuda-sdk --enable-cuvid --enable-libdrm --enable-ffplay --enable-gnutls --enable-gpl --enable-libass --enable-libfdk-aac --enable-libfontconfig --enable-libfreetype --enable-libmp3lame --enable-libnpp --enable-libopencore_amrnb --enable-libopencore_amrwb --enable-libopus --enable-libpulse --enable-sdl2 --enable-libspeex --enable-libtheora --enable-libtwolame --enable-libv4l2 --enable-libvorbis --enable-libvpx --enable-libx264 --enable-libx265 --enable-libxcb --enable-libxvid --enable-nonfree --enable-nvenc --enable-omx --enable-openal --enable-opencl --enable-runtime-cpudetect --enable-shared --enable-vaapi --enable-vdpau --enable-version3 --enable-xlib

### Sample command
cargo run -- --surah=1 --start-aya=1 --end-aya=1 --audio-type=urdu --text-type=none
