<table border="1">
    <caption>Packet Structure
    </caption>
    <tbody>
    <tr>
        <th colspan="2"><i>Offsets</i>
        </th>
        <th colspan="8">0
        </th>
        <th colspan="8">1
        </th>
        <th colspan="8">2
        </th>
        <th colspan="8">3
        </th>
    </tr>
    <tr>
        <th>Octet</th>
        <th>Bit
        </th>
        <th style="text-align:left;">0</th>
        <th>1</th>
        <th>2</th>
        <th>3</th>
        <th>4</th>
        <th>5</th>
        <th>6</th>
        <th>7</th>
        <th>0</th>
        <th>1
        </th>
        <th style="text-align:left;">2</th>
        <th>3</th>
        <th>4</th>
        <th>5</th>
        <th>6</th>
        <th>7</th>
        <th>0</th>
        <th>1</th>
        <th>2</th>
        <th>3
        </th>
        <th style="text-align:left;">4</th>
        <th>5</th>
        <th>6</th>
        <th>7</th>
        <th>0</th>
        <th>1</th>
        <th>2</th>
        <th>3</th>
        <th>4</th>
        <th>5
        </th>
        <th style="text-align:left;">6</th>
        <th>7
        </th>
    </tr>
    <tr>
        <th>0
        </th>
        <th>0
        </th>
        <td colspan="16">Source port</td>
        <td colspan="16">Destination port
        </td>
    </tr>
    <tr>
        <th>4
        </th>
        <th>32
        </th>
        <td colspan="32">Sequence number
        </td>
    </tr>
    <tr>
        <th>8
        </th>
        <th>64
        </th>
        <td colspan="32">Acknowledgment number (if ACK set)
        </td>
    </tr>
    <tr>
        <th>12
        </th>
        <th>96
        </th>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">ACK</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">WSC</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td colspan="24">Reserved</td>
    </tr>
    <tr>
        <th>
            16
        </th>
        <td>
            128
        </td>
        <td rowspan="3" colspan="32">Options</td>
    </tr>
    <tr>
        <td>...</td>
        <td>...</td>
    </tr>
    <tr>
        <td>28</td>
        <td>224</td>
    </tr>
    </tbody>
</table>




### Source port (16 bits)
identifies the sending port
### Destination port (16 bits)
identifies the receiving port

### Sequence number (32 bits)
this is the accumulated sequence number.
### Acknowledgment number (32 bits)
if the ACK flag is set then the value of this field is the next sequence number that the sender of the ACK is expecting. This acknowledges receipt of all prior bytes (if any). The first ACK sent by each end acknowledges the other end's initial sequence number itself, but no data.
### Data offset (4 bits)
Offset from the start of the segment to the actual data.
### Control bits (8 bits)
* ACK - Acknowledgment field significant
* WSC - Indicates that the Window resize option is present
* RES - Reserved for future use