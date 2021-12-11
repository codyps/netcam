

/// OD Cam stuff


/// "OD Cam". Internally referred to as "com.icatch.wificam"
const mcast_addr = std::net::Ipv4Addr::new(234, 168, 168, 168);
const mcast_port = 5002;



fn detect() {
    // 1. get mcast (android does funny stuff)
    // 2. bind socket to port
    // 3. join mcast group
    // 4. listen for packets (using 256 byte buffer)
    //
    // unclear if there is any packet content
}


/// ntk

enum AspectRatio {
    BestFit = 0,
    FullScreen = 1,
}

enum BlockingLevel {
    High = 3,
    Low = 1,
    Mid = 2,
    None = 0,
}

const device_ip = std::net::Ipv4Addr::new(192, 168, 1, 254);

const movie_url ="rtsp://192.168.1.254/xxx.mov";
const photo_url = "http://192.168.1.254:8192";
