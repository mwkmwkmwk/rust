// -*- rust -*-

impure fn a(chan[int] c) {
    c <| 10;
}

impure fn main() {
    let port[int] p = port();
    spawn a(chan(p));
    spawn b(chan(p));
    let int n = 0;
    n <- p;
    n <- p;
//    log "Finished.";
}

impure fn b(chan[int] c) {
//    log "task b0";
//    log "task b1";
//    log "task b2";
//    log "task b3";
//    log "task b4";
//    log "task b5";
    c <| 10;
}