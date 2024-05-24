import matplotlib.pyplot as plt
import csv

udp_x = []
udp_y = []

tcp_x = []
tcp_y = []

rrtp_x = []
rrtp_y = []

with open('udp.csv', 'r') as csvfile:
    plots = csv.reader(csvfile, delimiter=',')
    for row in plots:
        try:
            udp_y.append(int(row[0]))
            udp_x.append(int(row[1]))
        except:
            pass

with open('tcp.csv', 'r') as csvfile:
    plots = csv.reader(csvfile, delimiter=',')
    for row in plots:
        try:
            tcp_y.append(int(row[0]))
            tcp_x.append(int(row[1]))
        except:
            pass

with open('rrtp.csv', 'r') as csvfile:
    plots = csv.reader(csvfile, delimiter=',')
    for row in plots:
        try:
            rrtp_y.append(int(row[0]))
            rrtp_x.append(int(row[1]))
        except:
            pass


plt.plot(udp_x, udp_y, color ='r', label='UDP')
plt.plot(tcp_x, tcp_y, color ='b', label='TCP')
plt.plot(rrtp_x, rrtp_y, color ='g', label='RRTP')
# plt.xticks(rotation=25)
plt.xlabel('Time')
plt.ylabel('Cumulative Bytes')
plt.legend()
plt.savefig('plot.png')
plt.show()