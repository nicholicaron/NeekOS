# NeekOS
This project is my attempt to learn about Operating Systems by implementing one. As far as the name, my nickname is Neeko, so I figured the name is only right
- The initial code will draw heavily from Phil Opperman's blog series on the subject (https://os.phil-opp.com/). 
- Inspiration for how to expand from there will come from:
  - MIT's Caffeinated 6.828 (https://sipb.mit.edu/iap/6.828/),
  - and Georgia Tech's CS-3210 (https://tc.gtisc.gatech.edu/cs3210/2020/spring/info.html)

I will create a binary that will run on my Raspberry Pi 3B+, and will have at least the following features:
  - Shell
  - Bootloader
  - FAT32 Filesystem
  - Multitasking and Locking
  - Scalable TCP/IP

- Some further objectives include: 
  -  Implementing Address Space Layout Randomization
  -  Tracing memory allocations and deallocations
  -  Handling OOM (Out of Memory)
  -  Fuzzing NeekOS's interfaces/file systems
  -  Abstractions for Linux Device Drivers
  -  Linux Scheduling Policies
  -  Non-blocking synchronization (sleepable mutex)

Accompanying blog post coming soon
