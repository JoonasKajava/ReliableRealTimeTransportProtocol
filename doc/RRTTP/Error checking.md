UDP provides only checksum for error checking.

## Checksum
Checksum is used to verify that packet has not been corrupted during transit.

>[!info]
>Checksum only contains source address, destination address, zero, protocol, length
>It seems that it does not provide checksum for data it self.

## Sliding window

## References
- https://www.scs.stanford.edu/09au-cs144/notes/l2-print.pdf
- https://en.wikipedia.org/wiki/Internet_checksum
- https://www.packetmania.net/en/2021/12/26/IPv4-IPv6-checksum/