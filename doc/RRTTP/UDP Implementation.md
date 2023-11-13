Creating UDP socket is done using UdpSocket from rust standard library https://doc.rust-lang.org/std/net/struct.UdpSocket.html
This library is abstraction for operating system way of doing it.
On windows this is done using https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw. And on unix/linux systems its done using libc (didn't look for more specific function)

>[!warning]
>It's unlikely I will create my own implementation for this as it's a bit out of scope for this project.

 
## Packet 
UdpSocket creates udp-packet that seems to contain checksum
![[Udp-packet.png]]

