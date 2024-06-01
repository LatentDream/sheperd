default:
  just --list

bin := "./target/release/balsam"

build:
    RUSTFLAGS='-g' cargo build --release

cpu-profile:
    valgrind --tool=callgrind --callgrind-out-file=callgrind.out --collect-jumps=yes --simulate-cache=yes {{bin}}

cpu-profile-check:
    kcachegrind callgrind.out

benchmark:
    hyperfine {{bin}}

mem-check:
    valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes {{bin}}

mem-profile:
    heaptrack -o /tmp/heaptrack.pbqdd.%p.zst {{bin}}

mem-profile-check:
    heaptrack_gui "/tmp/heaptrack.pbqdd.%p.zst.gz"
