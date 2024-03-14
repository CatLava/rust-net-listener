use pcap::{Device, Capture};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;



fn main() {
    println!("Hello, world!");
    let devices = Device::list().unwrap();
    // need to pick the correct device
    // monitor for packets on device 
    // use pnet library 
    for i in devices{

        println!("{:?}", i.name);
        if i.name == "en8" {
            let mut cap = Capture::from_device(i).unwrap()
                .promisc(true)
                .open()
                .unwrap();
            //cap.filter("tcp", true).unwrap();
            while let Ok(packet) = cap.next_packet() {
                let ethernet_packet = Ipv4Packet::new(&packet).ok_or("Failed to parse Ethernet packet").expect("Notpacker");
                println!("received packet! to {:?} ; from {:?}", ethernet_packet.get_destination(), ethernet_packet.get_source());
                // if let IpNextHeaderProtocols::Ipv4 = ethernet_packet.expect("NOt PACKET").get_ethertype() {
                //     println!("IPV4")
                // } else {
                //     println!("NOT")
                // }


            }
        }


    }   
    let main_device = Device::lookup().unwrap().unwrap();
    println!("Main Device {:?}", main_device);
    let mut cap = Capture::from_device(main_device).unwrap()
                    .promisc(true)
                    .open()
                    .unwrap();
    println!("next {:?}", cap.next_packet());
    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }
    // will use main device 

}
