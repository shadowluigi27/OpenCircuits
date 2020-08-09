*12V DC Voltage source (Note: node #0 = ground)

Vsource vin 0 DC 12

*R1 = 1k, R2 = 470 Ohms

R1 vin vout 1k

R2 vout 0 470

.control

tran .5s 1s; transient analysis (pg. 238 ngspice manual)

.endc

.end
