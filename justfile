last := `ls src/day* | sort | tail -n 1 | awk -F '[/\.]' '{print $2}'`

#[default]
test filter=last:
    cargo test {{filter}}
