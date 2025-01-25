<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/">
    <img src="images/logo.png" alt="Logo" width="480" height="240">
  </a>

  <h3 align="center">NeekOS</h3>

  <p align="center">
    This project is my attempt to learn about Operating Systems by implementing one. 
    <br />
    <a href="https://github.com/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/">View Demo</a>
    &middot;
    <a href="https://github.com//issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com//issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]]()

NeekOS is a bare metal kernel that demonstrates fundamental operating system concepts through clean, well-documented Rust implementations. It runs on x86_64 architecture and includes essential OS components like memory management, interrupts handling, and keyboard input.

### Key Features

- **Bare Metal x86_64**: Runs directly on hardware with no underlying OS
- **Memory Management**:
    - Virtual Memory Management with 4-level paging
    - Physical Frame Allocation
    - Multiple Heap Allocator implementations:
        - Bump Allocator (simple but fast)
        - Linked List Allocator (handles fragmentation better)
        - Fixed Size Block Allocator (optimized for common allocation sizes)
- **Interrupt Handling**:
    - Custom Interrupt Descriptor Table (IDT)
    - Hardware Interrupt support (PIC8259)
    - Configurable interrupt handlers
    - Double Fault handling with separate stack
- **Hardware Support**:
    - VGA text mode output
    - PS/2 Keyboard input
    - Serial port communication
    - PIT Timer: System timing
- **Testing Framework**:
    - Custom test runner
    - Integration tests
    - Panic and stack overflow tests
    - Support for QEMU testing environment


<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With


* [![Rust][Rust]][Rust-url]


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

Note: Some instructions (Qemu) assumes you are using Arch Linux. Please adjust
accordingly for your platform

* Install Rust Toolchain
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
  ```
* Install Rust Nightly
  ```sh
  rustup override set nightly
  ```
* Install Required components
  ```sh
  rustup component add rust-src llvm-tools-preview
  ```
* Install Bootimage
  ```sh
  cargo install bootimage
  ```
* Install Qemu, and git
  ```sh
  sudo pacmann -S qemu-full git
  ```

### Installation

* Clone the repository:
```sh
git clone https://github.com/nicholicaron/NeekOS.git
```

* Build the kernel:
```sh
cargo bootimage
```

* Run the kernel (in Qemu)
```sh
cargo run
```


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] Create Freestanding, Minimal Rust Kernal
- [x] Print to Screen using VGA text buffer
- [x] Implement standalone testing framework
- [x] Handle CPU Exceptions
- [x] Handle Hardware Interrupts
- [x] Set up paging
- [x] Set up dynamic memory allocation on the heap
- [ ] Configure cooperative multitasking
- [ ] Fat32 File system support
- [ ] Process Scheduling
- [ ] User space programs
- [ ] TCP Network Stack

### Longshot goals
- [ ] Implementing Address Space Layout Randomization
- [ ] Tracing memory allocations and deallocations
- [ ] Handling OOM (Out of Memory)
- [ ] Fuzzing NeekOS's interfaces/file systems
- [ ] Abstractions for Linux Device Drivers
- [ ] Linux Scheduling Policies
- [ ] Non-blocking synchronization (sleepable mutex)


See the [open issues](https://github.com/nicholicaron/NeekOS/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Use this space to list resources you find helpful and would like to give credit to. I've included a few of my favorites to kick things off!

- The initial code will draw heavily from Phil Opperman's [blog series on the subject](https://os.phil-opp.com/). 
- Inspiration for how to expand from there will come from:
  - MIT's [Caffeinated 6.828](https://sipb.mit.edu/iap/6.828/),
  - and Georgia Tech's [CS-3210](https://tc.gtisc.gatech.edu/cs3210/2020/spring/info.html)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[forks-shield]: https://img.shields.io/github/forks/nicholicaron/NeekOS.svg?style=for-the-badge
[forks-url]: https://github.com/nicholicaron/NeekOS/network/members
[stars-shield]: https://img.shields.io/github/stars/nicholicaron/NeekOS.svg?style=for-the-badge
[stars-url]: https://github.com/nicholicaron/NeekOS/stargazers
[issues-shield]: https://img.shields.io/github/issues/nicholicaron/NeekOS.svg?style=for-the-badge
[issues-url]: https://github.com/nicholicaron/NeekOS/issues
[license-shield]: https://img.shields.io/github/license/nicholicaron/NeekOS.svg?style=for-the-badge
[license-url]: https://github.com//blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
[Rust]: https://shields.io/badge/-Rust-3776AB?style=flat&logo=rust
[Rust-url]: https://www.rust-lang.org/
