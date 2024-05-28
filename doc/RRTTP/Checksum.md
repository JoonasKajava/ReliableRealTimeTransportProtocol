
https://learn.microsoft.com/en-us/windows-server/networking/technologies/hpn/hpn-hardware-only-features

Is seems that windows offloads checksum calculation to Network Interface Controller.

>[!info]
the checksum offload calculates the checksums in the IP, TCP, and UDP headers (as appropriate) and indicates to the OS whether the checksums passed, failed, or not checked. If the NIC asserts that the checksums are valid, the OS accepts the packet unchallenged. If the NIC asserts the checksums are invalid or not checked, the IP/TCP/UDP stack internally calculates the checksums again. If the computed checksum fails, the packet gets discarded.

>[!warning]
> Network interface controller handles checksums.
> https://learn.microsoft.com/en-us/windows-server/networking/technologies/hpn/hpn-hardware-only-features