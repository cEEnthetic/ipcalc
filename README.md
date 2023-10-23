# ipcalc
remaking my friend's project in rust

A simple ip calculator, featuring:
- the calculator itself, which features:
    - network/broadcast calculator
    - indexing in a network
    - number of hosts calculator
- cli interface, which features:
    - shorthand for ip input (eg.: 192.168.100 == 192.168.100.0)
    - support for prefixed ip input (192.168.100.1/24)
    - finding hosts by index
    - output, outputting: mask, index, network, broadcast, number of hosts

    - future features:
        - shorthand for broadcast addresses
        - ip binary output
        - coloring output network and host bits
        - support for binary and dotted binary input
