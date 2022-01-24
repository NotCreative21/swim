killall swim
killall Xephyr
sleep 2
cargo build && Xephyr -screen 1280x720 :1 &
sleep 1
DISPLAY=:1 ./target/debug/swim &
sleep 1
DISPLAY=:1 st &
