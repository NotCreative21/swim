killall swim
killall Xephyr
sleep 2
cargo build --release && Xephyr -screen 1280x720 :1 &
sleep 1
DISPLAY=:1 xrdb ~/.config/Xresources &
DISPLAY=:1 ./target/release/swim &
sleep 1
DISPLAY=:1 st &
