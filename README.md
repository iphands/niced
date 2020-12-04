# niced
A k.i.s.s. renicing daemon for Linux

## Setup
`niced` required a conf file at `/etc/niced.conf`.


The conf file format is also k.i.s.s.
A simple set of `comm=nice_val\n` (String=int\n)
```
qemu=-19
X=-10
compton=-10
fluxbox=-10
pulseaudio=-6
chrome=-5
nacl=-5
term=-5
emacs=-5
slack=1
sshd=1
niced=19
rust_tray=19
notification-da=19
lxdm-binary=19
conky=19
xosview=19
randy=19
```
