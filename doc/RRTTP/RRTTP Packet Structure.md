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
        <th>32
        </th>
        <td colspan="32">Sequence number
        </td>
    </tr>
    <tr>
        <th>4
        </th>
        <th>64
        </th>
        <td colspan="32">Acknowledgment number (if ACK set)
        </td>
    </tr>
    <tr>
        <th>8
        </th>
        <th>96
        </th>
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">ACK</span>
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
        <td><span
                style="writing-mode: vertical-lr; text-orientation: upright; letter-spacing: -0.12em; line-height:1em; width:1em;">RES</span>
        </td>
        <td colspan="16">Segmented Data Identifier</td>
        <td colspan="8">Data Offset</td>
    </tr>
    <tr>
        <th>
            12
        </th>
        <td>
            128
        </td>
        <td rowspan="2" colspan="32">Options</td>
    </tr>
    <tr>
        <td>...</td>
        <td>...</td>
    </tr>
    <tr>
        <td>24</td>
        <td>192</td>
        <td rowspan="2" colspan="32">
Data
</td>
</tr>
    <tr>
        <td>...</td>
        <td>...</td>
    </tr>
<tr>
<td>152</td>
<td>1216</td>
</tr>
    </tbody>
</table>

### Sequence number (32 bits)

this is the accumulated sequence number.

### Acknowledgment number (32 bits)

if the ACK flag is set then the value of this field is the next sequence number that the sender of the ACK is expecting.
This acknowledges receipt of all prior bytes (if any). The first ACK sent by each end acknowledges the other end's
initial sequence number itself, but no data.

### Control bits (8 bits)

* ACK - Acknowledgment field significant
* RES - Reserved for future use

### Segmented Data Identifier (16 bits)

Identifies the segmented data.

### Data offset (4 bits)

Offset from the start of the packet to the actual data.

### Options (Variable 0-96 bits, in units of 48 bits)

<table border="1">
<tr>
<th>
Option Kind (8 bits)
</th>
<th>
Option Length (8 bits)
</th>
<th>
Option Data (Variable based on Option Length)
</th>
</tr>
<tr>
<td>
BufferSize (1)
</td>
<td>
2 
</td>
<td>
Size of the buffer presented as a 32 bit unsigned integer.
</td>
</tr>
<tr>
<td>Segment Number</td>
<td>2</td>
<td>Segment number presented as a 32 bit unsigned integer.</td>
</tr>
</table>