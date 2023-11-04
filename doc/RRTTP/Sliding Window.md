Sliding window ensures that packets are not lost in transit and it will be used to order packets.

The time that it takes for the ACK signal to be received may represent a significant amount of time compared to the time needed to send the packet. In this case, the overall [throughput](https://en.wikipedia.org/wiki/Throughput "Throughput") may be much lower than theoretically possible. To address this, sliding window protocols allow a selected number of packets, the _window_, to be sent without having to wait for an ACK. Each packet receives a sequence number, and the ACKs send back that number. The protocol keeps track of which packets have been ACKed, and when they are received, sends more packets. In this way, the window _slides_ along the stream of packets making up the transfer.

>[!warning]
>For the highest possible [throughput](https://en.wikipedia.org/wiki/Throughput "Throughput"), it is important that the transmitter is not forced to stop sending by the sliding window protocol earlier than one [round-trip delay time](https://en.wikipedia.org/wiki/Round-trip_delay_time "Round-trip delay time") (RTT). The limit on the amount of data that it can send before stopping to wait for an acknowledgment should be larger than the [bandwidth-delay product](https://en.wikipedia.org/wiki/Bandwidth-delay_product "Bandwidth-delay product") of the communications link. If it is not, the protocol will limit the effective [bandwidth](https://en.wikipedia.org/wiki/Bandwidth_(computing) "Bandwidth (computing)") of the link.

## Operation
The transmitter and receiver each have a current sequence number _n<sub>t</sub>_ and _n<sub>r</sub>_, respectively. They each also have a window size _w<sub>t</sub>_ and _w<sub>r</sub>_. The window sizes may vary, but in simpler implementations they are fixed. The window size must be greater than zero for any progress to be made.

As typically implemented, _n<sub>t</sub>_ is the next packet to be transmitted, i.e. the sequence number of the first packet not yet transmitted. Likewise, _n<sub>r</sub>_ is the first packet not yet received. Both numbers are [monotonically increasing](https://en.wikipedia.org/wiki/Monotonically_increasing "Monotonically increasing") with time; they only ever increase.

## Acknowledgement (data networks)
[ASCII](https://en.wikipedia.org/wiki/ASCII "ASCII") code includes an ACK character (0000110<sub>2</sub> or 6<sub>16</sub>) which can be transmitted to indicate successful receipt and a NAK character (0010101<sub>2</sub> or 15<sub>16</sub>) which can be transmitted to indicate an inability or failure to receive.

The TCP protocol allows these acknowledgements to be included with data that is sent in the opposite direction.

Some protocols send a single acknowledgement per packet of information. Other protocols such as TCP and [ZMODEM](https://en.wikipedia.org/wiki/ZMODEM "ZMODEM") allow many packets to be transmitted before receiving acknowledgement for any of them, a procedure necessary to fill high [bandwidth-delay product](https://en.wikipedia.org/wiki/Bandwidth-delay_product "Bandwidth-delay product") links with a large number of bytes in flight.
## Packet
each portion of the transmission (packets in most data link layers, but bytes in TCP) is assigned a unique consecutive sequence number, and the receiver uses the numbers to place received packets in the correct order, discarding duplicate packets and identifying missing ones.

By placing limits on the number of packets that can be transmitted or received at any given time, a sliding window protocol allows an unlimited number of packets to be communicated using fixed-size sequence numbers.
## Window

The size of the sending window determines the sequence number of the outbound frames. If the sequence number of the frames is an n-bit field, then the range of sequence numbers that can be assigned is 0 to 2^𝑛−1. Consequently, the size of the sending window is 2^𝑛−1. Thus in order to accommodate a sending window size of 2^𝑛−1, a n-bit sequence number is chosen.
## Ordering
Each packet must have unique sequence number that will be used to reorder packets.

## Notes
- When the receiver verifies the data, it sends an [acknowledgment signal](https://en.wikipedia.org/wiki/Acknowledgement_(data_networks) "Acknowledgement (data networks)"), or "ACK", back to the sender to indicate it can send the next packet.
- The TCP header uses a 16 bit field to report the receiver window size to the sender. Therefore, the largest window that can be used is 216 = 64 kilobytes.
- It is possible to not acknowledge every packet, as long as an acknowledgment is sent eventually if there is a pause. For example, TCP normally acknowledges every second packet.

## Automatic Repeat Request (ARQ)
In a simple [automatic repeat request](https://en.wikipedia.org/wiki/Automatic_repeat_request "Automatic repeat request") protocol (ARQ), the sender stops after every packet and waits for the receiver to ACK. This ensures packets arrive in the correct order, as only one may be sent at a time.
### Stop-and-wait ARQ
Also referred to as alternating bit protocol.  It is the simplest automatic repeat-request (ARQ) mechanism.  A stop-and-wait ARQ sender sends one [frame](https://en.wikipedia.org/wiki/Frame_(telecommunications) "Frame (telecommunications)") at a time; it is a special case of the general [sliding window protocol](https://en.wikipedia.org/wiki/Sliding_window_protocol "Sliding window protocol") with transmit and receive window sizes equal to one in both cases.  After sending each frame, the sender doesn't send any further frames until it receives an [acknowledgement](https://en.wikipedia.org/wiki/Acknowledgement_(data_networks) "Acknowledgement (data networks)") (ACK) signal. After receiving a valid frame, the receiver sends an ACK. If the ACK does not reach the sender before a certain time, known as the timeout, the sender sends the same frame again. The timeout countdown is reset after each frame transmission. 
>[!warning]
> Stop-and-wait ARQ is inefficient compared to other ARQs.

Because the time between packets, if the ACK and the data are received successfully, is twice the transit time (assuming the [turnaround time](https://en.wikipedia.org/wiki/Turnaround_time "Turnaround time") can be zero). The throughput on the channel is a fraction of what it could be. To solve this problem, one can send more than one packet at a time with a larger sequence number and use one ACK for a set. This is what is done in [Go-Back-N ARQ](https://en.wikipedia.org/wiki/Go-Back-N_ARQ "Go-Back-N ARQ") and the [Selective Repeat ARQ](https://en.wikipedia.org/wiki/Selective_Repeat_ARQ "Selective Repeat ARQ").
### Go-Back-N ARQ
**Go-Back-_N_ ARQ** is a specific instance of the [automatic repeat request](https://en.wikipedia.org/wiki/Automatic_repeat_request "Automatic repeat request") (ARQ) protocol, in which the sending process continues to send a number of [frames](https://en.wikipedia.org/wiki/Data_frame "Data frame") specified by a _window size_ even without receiving an [acknowledgement](https://en.wikipedia.org/wiki/Acknowledgement_(data_networks) "Acknowledgement (data networks)") (ACK) packet from the receiver. It is a special case of the general [sliding window protocol](https://en.wikipedia.org/wiki/Sliding_window_protocol "Sliding window protocol") with the transmit window size of N and receive window size of 1. It can transmit N frames to the peer before requiring an ACK.

The receiver process keeps track of the sequence number of the next frame it expects to receive. It will discard any frame that does not have the exact sequence number it expects (either a duplicate frame it already acknowledged, or an out-of-order frame it expects to receive later) and will send an ACK for the last correct in-order frame. Once the sender has sent all of the frames in its _window_, it will detect that all of the frames since the first lost frame are _outstanding_, and will go back to the sequence number of the last ACK it received from the receiver process and fill its window starting with that frame and continue the process over again.
> [!warning]
> However, this method also results in sending frames multiple times – if any frame was lost or damaged, or the ACK acknowledging them was lost or damaged, then that frame and all following frames in the send window (even if they were received without error) will be re-sent. To avoid this, [Selective Repeat ARQ](https://en.wikipedia.org/wiki/Selective_Repeat_ARQ "Selective Repeat ARQ") can be used

### Selective Repeat ARQ
Selective Repeat is part of the automatic repeat request (ARQ). With selective repeat, the sender sends a number of frames specified by a window size even without the need to wait for individual ACK from the receiver as in [Go-Back-N ARQ](https://en.wikipedia.org/wiki/Go-Back-N_ARQ "Go-Back-N ARQ"). The receiver may selectively reject a single frame, which may be retransmitted alone; this contrasts with other forms of ARQ, which must send every frame from that point again. The receiver accepts out-of-order frames and buffers them. The sender individually retransmits frames that have timed out.

The receiver process keeps track of the sequence number of the earliest frame it has not received, and sends that number with every [acknowledgement](https://en.wikipedia.org/wiki/Acknowledgement_(data_networks) "Acknowledgement (data networks)") (ACK) it sends. If a frame from the sender does not reach the receiver, the sender continues to send subsequent frames until it has emptied its _window_. The receiver continues to fill its receiving window with the subsequent frames, replying each time with an ACK containing the sequence number of the earliest missing [frame](https://en.wikipedia.org/wiki/Data_frame "Data frame"). Once the sender has sent all the frames in its _window_, it re-sends the frame number given by the ACKs, and then continues where it left off.
>[!info] 
>The size of the sending and receiving windows must be equal, and half the maximum sequence number (assuming that sequence numbers are numbered from 0 to _n_−1) to avoid miscommunication in all cases of packets being dropped

If the receiving window is larger than half the maximum sequence number, some, possibly even all, of the packets that are present after timeouts are duplicates that are not recognized as such. The sender moves its window for every packet that is acknowledged.

When used as the protocol for the delivery of **subdivided messages** it works somewhat differently. In non-continuous channels where messages may be variable in length, standard ARQ or Hybrid ARQ protocols may treat the message as a single unit. Alternately selective retransmission may be employed in conjunction with the basic ARQ mechanism where the message is first subdivided into sub-blocks (typically of fixed length) in a process called [packet segmentation](https://en.wikipedia.org/wiki/Packet_segmentation "Packet segmentation")

>[!info]
>in ARQ with selective transmission the ACK response would additionally carry a bit flag indicating the identity of each sub-block successfully received.
## Packet Segmentation
In data communications networks, packet segmentation is the process of dividing a data packet into smaller units for transmission over the network.

Protocols that perform packet segmentation at the source usually include a mechanism at the destination to reverse the process and reassemble the original packet from individual segments. This process may include [automatic repeat-request](https://en.wikipedia.org/wiki/Automatic_repeat-request "Automatic repeat-request") (ARQ) mechanisms to detect missing segments and to request the source to re-transmit specific segments.

## References
- https://www.baeldung.com/cs/tcp-flow-control-vs-congestion-control
- https://github.com/ffahleraz/sliding-window-protocol/tree/master
- https://www.tutorialspoint.com/sliding-window-protocol
- https://en.wikipedia.org/wiki/Sliding_window_protocol