             extern crate bincode;
             extern crate serde;
#[macro_use] extern crate serde_derive;

use std::net::{UdpSocket, SocketAddr};
use std::time::{Instant, Duration};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8413").unwrap();

    let mut requesters: Vec<Requester> = Vec::with_capacity(10000);
    let mut cleanup = Instant::now();

    let dur10 = Duration::new(10, 0);
    let dur30 = Duration::new(30, 0);

    loop {
        let mut buf = [0; 1024];
        if let Ok((_, addr)) = socket.recv_from(&mut buf) {
            // remove outdated requesters
            if cleanup.elapsed() > dur10 {
                cleanup = Instant::now();
                requesters.retain(|x| cleanup.duration_since(x.timestamp) < dur30);
            }

            // process message
            match buf[0] {
                0x00 => {
                    println!("Received message {}", addr);
                    if let Ok(request) = bincode::deserialize::<MatchMakingRequest>(&buf[1..]) {
                        let mut new_request = true;

                        // if already requested then only refresh the request
                        for requester in requesters.iter_mut() {
                            if requester.address == addr {
                                requester.match_making_request = request.clone();
                                requester.timestamp = Instant::now();
                                new_request = false;
                                println!("Refresh");
                                break;
                            }
                        }

                        if new_request {
                            let mut matching_requesters = vec!();

                            // search for matching requesters
                            let mut matched = false;
                            for (i, check_requester) in requesters.iter().enumerate() {
                                if check_requester.match_making_request == request {
                                    matching_requesters.push(i);
                                    if matching_requesters.len() as u8 + 1 == request.num_players {
                                        matched = true;
                                        break
                                    }
                                }
                            }

                            if matched {
                                // respond to the new requester
                                let addresses = matching_requesters.iter().map(|i| &requesters[*i].address).cloned().collect();
                                respond(addr, addresses, &socket);

                                // respond to all matching requesters
                                for requester_i in matching_requesters.iter() {
                                    let requester_address = requesters[*requester_i].address;
                                    let mut addresses: Vec<SocketAddr> = matching_requesters.iter()
                                        .map(|i| requesters[*i].address.clone())
                                        .filter(|x| x != &requester_address)
                                        .collect();
                                    addresses.push(addr);
                                    respond(requester_address, addresses, &socket);
                                }

                                // remove all matching requesters
                                matching_requesters.reverse();
                                for i in matching_requesters {
                                    requesters.remove(i);
                                }
                            }
                            else {
                                // store the new requester, so it can be matched later
                                requesters.push(Requester {
                                    address:              addr,
                                    timestamp:            Instant::now(),
                                    match_making_request: request
                                });
                                println!("Store");
                            }
                        }
                    }
                }
                _ => {
                    println!("Couldn't process netplay message starting with: {:?}", &buf[0..32]);
                }
            }
        }
    }
}


fn respond(send_to: SocketAddr, addresses: Vec<SocketAddr>, socket: &UdpSocket) {
    let response = MatchMakingResponse { addresses };
    let mut data = bincode::serialize(&response).unwrap();
    data.insert(0, 0x00);
    socket.send_to(&data, send_to).unwrap();
    println!("Sent message to {}", send_to);
}

struct Requester {
    address:              SocketAddr,
    timestamp:            Instant,
    match_making_request: MatchMakingRequest,
}

#[derive(Clone, PartialEq, Deserialize)]
struct MatchMakingRequest {
    region:        String,
    package_hash:  String,
    build_version: String,
    num_players:   u8
}

#[derive(Clone, Serialize)]
struct MatchMakingResponse {
    addresses: Vec<SocketAddr>
}
